use tokio::{
    sync::broadcast
};
use warp::{
    Filter, 
    ws::{Message, WebSocket}
};
use futures::{
    StreamExt, SinkExt
};
use std::{
    io::{
        self,
        Write
    },
    fs,
    time::{
        Duration
    },
    process::{Command}, 
    sync::{
        mpsc, Arc, Mutex
    },
    thread
};
use serde_json::{
    json
};

use rusb::{
    DeviceHandle, 
    GlobalContext,
    open_device_with_vid_pid, 
    Direction, 
    TransferType
};


use classes::A_o_input_device;
use classes::A_o_name_synonym;
pub mod classes;
pub mod functions;

use functions::{
    f_update_o_input_device
};
use classes::{
    ControlCommand, O_input_device, SendData
};

use gethostname::gethostname;

fn f_usb_read_thread(
    o_tx_control_usb: mpsc::Sender<SendData>,
    o_rx_control_usb: mpsc::Receiver<ControlCommand>, 
) {

    let s_json = fs::read_to_string("./o_info.json").expect("Unable to read file");
    // Deserialize the JSON data into a serde_json::Value
    let v: serde_json::Value = serde_json::from_str(&s_json).expect("cannot parse json");
    let mut a_o_input_device: A_o_input_device = serde_json::from_value(v.get("a_o_input_device").expect("json must have a_o_input_device").clone()).expect("cannot decode json");
    let mut a_o_name_synonym: A_o_name_synonym = serde_json::from_value(v.get("a_o_name_synonym").expect("json must have a_o_name_synonym").clone()).expect("cannot decode json");
    println!("a_o_input_device {:?}", a_o_input_device);
    let mut v_o_input_device: Option<&mut O_input_device> = None;
    let mut o_device_handle_option: Option<DeviceHandle<GlobalContext>> = None;
    let mut b_usb_readout_active = false;
    let mut n_id_vendor = 0;
    let mut n_id_product = 0;
    let mut o_timeout = Duration::from_millis(1000);
    let mut n_len_a_n_u8__readout = 32;
    let mut n_interface = 0;
    let mut n_address_endpoint_in = 0x81;
    let n_class_code_human_interface_device = 3;
    loop {
        // Check for control commands
        match o_rx_control_usb.try_recv() {
            Ok(ControlCommand::Start) => {
                b_usb_readout_active = true;
            },
            Ok(ControlCommand::Stop) => {
                b_usb_readout_active = false;
            },
            Ok(ControlCommand::SwitchDevice(vid, pid)) => {
                if(o_device_handle_option.is_some()){
                    println!("release old iface and attach kernel driv");
                    if let Some(ref mut o_device_handle) = o_device_handle_option{
                        let _ = o_device_handle.release_interface(n_interface).expect("cannot release interface");
                        // let _ = o_device_handle.attach_kernel_driver(0).expect("cannot attach kernel driver");
                    }
                    // release the old interface and attach the kernel driver
                }

                n_id_vendor = vid;
                n_id_product = pid;
                v_o_input_device = a_o_input_device
                    .iter_mut()
                    .find(
                        |o| o.n_id_vendor == n_id_vendor && o.n_id_product == n_id_product
                    );

                o_device_handle_option = open_device_with_vid_pid(vid, pid);
                match o_device_handle_option{
                    Some(ref mut o_device_handle)=>{
                        b_usb_readout_active = true;

                        // find a interface with a interrupt IN, mostly this will be index 0 but on dualsense for example it is 3
                        println!("Changing from old vid:pid 0x{:02x}:0x{:02x} to vid:pid 0x{:02x}:0x{:02x}", n_id_vendor, n_id_product, vid, pid);
                        
                        let o_config_descriptor = o_device_handle.device().active_config_descriptor().expect("Failed to get configuration descriptor");
                        let mut b_interface_found: bool = false;
                        for o_interface in o_config_descriptor.interfaces() {
                    
                            for o_interface_descriptor in o_interface.descriptors() {
                                for o_endpoint_descriptor in o_interface_descriptor.endpoint_descriptors() {
                                    if o_endpoint_descriptor.direction() == Direction::In
                                        && o_endpoint_descriptor.transfer_type() == TransferType::Interrupt
                                    {
                                        if(o_interface_descriptor.class_code() == n_class_code_human_interface_device){
                    
                                            println!("Found an IN endpoint: 0x{:02x}", o_endpoint_descriptor.address());
                                            n_address_endpoint_in = o_endpoint_descriptor.address();
                                            let n = o_endpoint_descriptor.interval() as f32;
                                            n_interface = o_interface.number();
                                            o_timeout = Duration::from_millis((n*5.00) as u64);// account for overhead and ensure reliability
                                            n_len_a_n_u8__readout = o_endpoint_descriptor.max_packet_size();
                                            b_interface_found = true;
                                            
                                        }
                                        if(b_interface_found){break;}
                                    }
                                    if(b_interface_found){break;}
                                }
                                if(b_interface_found){break;}
                            }
                            if(b_interface_found){break;}
                        }
                        println!(
                            "found interrupt interface: 
                            {}, 
                            endpoint IN 
                            addr:{}, 
                            timeout:{:?}, 
                            n_len_a_n_u8_readout {:?}
                            ", n_interface,
                            n_address_endpoint_in,
                            o_timeout,
                            n_len_a_n_u8__readout
                        );


                        // if no suitable interface is found the 'default' values will be used
                            let _ = o_device_handle.set_auto_detach_kernel_driver(true).expect("cannot set auto a- de- tach of the kernel driver");
                            let _ = o_device_handle.claim_interface(n_interface).expect("Cannot claim interface");
                    }
                    None =>{
                        println!("cannot open usb device, is it connected and are you root?");
                        b_usb_readout_active = false;
                    }
                }
                


            },
            Err(mpsc::TryRecvError::Empty) => {},
            Err(_) => break,  // Exit on other errors
        }

        if b_usb_readout_active {
            // println!("readout?");
            let mut a_n_u8__readout = vec![0u8; n_len_a_n_u8__readout as usize];
            if let Some(ref mut o_device_handle) = o_device_handle_option{
                // println!("readout?");
                match o_device_handle.read_interrupt(n_address_endpoint_in, &mut a_n_u8__readout, o_timeout) {
                    Ok(n_bytes_read) => {
                        let mut v_o_input_device_cloned = None;
                        if let Some(ref mut o_input_device) = v_o_input_device{
                            v_o_input_device_cloned = Some(o_input_device.clone()); 
                            f_update_o_input_device(
                                o_input_device, 
                                &a_n_u8__readout
                            );
                            // print!("\x1B[2J\x1B[H"); // Clear the screen and move the cursor to the top-left
                            // io::stdout().flush().unwrap();
                        
                            for o_input_sensor in &o_input_device.a_o_input_sensor{

                                println!("{:?}:{:?} {:?}", o_input_sensor.s_name, o_input_sensor.n_nor, o_input_sensor.v_o_num_str_value);
                            }
                        }
                        // println!("Read from USB device success, bytes read: {:?}", a_n_u8__readout);
                        o_tx_control_usb.send(SendData{
                            a_n_u8_usb_read_result: Some(a_n_u8__readout),
                            v_o_input_device: v_o_input_device_cloned
                        }).expect("Failed to send data to main thread");

                        // Send data to the main application if needed
                    },
                    Err(e) => {
                        eprintln!("USB read error: {:?}", e);
                    }
                }
            }

        }

        // Sleep to prevent tight looping
        thread::sleep(Duration::from_millis(10));
    }
}

async fn f_websocket_thread(
    mut o_websocket: WebSocket,
    mut o_rx_receiver: broadcast::Receiver<String>, 
    mut o_tx_sender_clone: broadcast::Sender<String>, 
    o_tx_control_usb: mpsc::Sender<ControlCommand>, 
) { 
    let (mut o_ws_sender, mut o_ws_receiver) = o_websocket.split();

    // Task to forward broadcast messages to WebSocket client
    let forward_task = tokio::spawn(async move {
        while let Ok(s_msg) = o_rx_receiver.recv().await {
            let o_msg = Message::text(s_msg);
            if o_ws_sender.send(o_msg).await.is_err() {
                eprintln!("Failed to send WebSocket message");
                break;
            }
        }
    });

    // Optionally handle incoming messages
    while let Some(result) = o_ws_receiver.next().await {
        if let Ok(msg) = result {
            if msg.is_close() {
                break;
            }
            if let Ok(text) = msg.to_str() {
                println!("websocket received message");
                if let Ok(v_json_parsed) = serde_json::from_str::<serde_json::Value>(text) {

                    if let Some(s_name_function) = v_json_parsed.get("s_name_function").and_then(|v: &serde_json::Value| v.as_str()) {
                        println!("received s_name_function {}", s_name_function);
                        o_tx_sender_clone.send(String::from("message received, heres a new one sent!"));

                        if(s_name_function == "f_s_stdout_from_s_command"){
                            if let Some(s_command) = v_json_parsed.get("s_command").and_then(|v: &serde_json::Value| v.as_str()) {

                                let a_s_arg :Vec<&str> = s_command.split_whitespace().collect();
                                let a_s_command_allowed = ["lsusb", "touch lol_test"];
                                if !a_s_command_allowed.contains(&s_command) {
                                    let sresp = format!("command '{}' not allowed, allowed are {:?}", s_command, a_s_command_allowed);
                                    o_tx_sender_clone.send(sresp);
                                }else{
                                    let a_s_arg2 = if a_s_arg.len() > 1 { &a_s_arg[1..] } else { &[] };
                                    let s_out = Command::new(a_s_arg[0])
                                        .args(a_s_arg2)
                                        .output()
                                        .expect("failed to execute process");
                                    println!("s_out {:?}", s_out);
            
                                    // Convert output to a string and send it back
                                    let s_stdout = String::from_utf8_lossy(&s_out.stdout);

                                    o_tx_sender_clone.send(json!({ "s_stdout__lsusb": s_stdout }).to_string());
                                            // println!("message {:?}", sdev);
                                    // i want to be able to send data here... 
                                }
                            }
                        }
                        if(s_name_function == "f_switch_usb_device"){

                            let n_id_vendor = v_json_parsed.get("n_id_vendor").unwrap().as_i64().expect("value has to be number");
                            let n_id_product = v_json_parsed.get("n_id_product").unwrap().as_i64().expect("value has to be number");

                            o_tx_control_usb.send(ControlCommand::Stop).unwrap();
                            o_tx_control_usb.send(ControlCommand::SwitchDevice(
                                n_id_vendor as u16, 
                                n_id_product as u16, 
                            )).unwrap();
                            o_tx_control_usb.send(ControlCommand::Start).unwrap();
                        }
                        
                    }
                }
            }
        }

    }

    // Make sure the forwarding task is properly cleaned up
    let _ = forward_task.await;
}

#[tokio::main]
async fn main() {
    let (o_tx_control_usb1, o_rx_control_usb1) = mpsc::channel();
    let (o_tx_control_usb2, o_rx_control_usb2) = mpsc::channel();

    thread::spawn(move || {
        f_usb_read_thread(o_tx_control_usb2, o_rx_control_usb1);
    });

    let n_messages_at_a_time_in_the_channel_buffer_max = 10;
    let (o_tx_sender, _) = broadcast::channel::<String>(n_messages_at_a_time_in_the_channel_buffer_max);
    let o_tx_sender_clone = o_tx_sender.clone();
    let routes = warp::path("ws")
        .and(warp::ws())
        .map(move |o_ws: warp::ws::Ws| {
            let o_rx_receiver: broadcast::Receiver<String> = o_tx_sender.subscribe();
            let o_tx_sender_clone = o_tx_sender.clone();
            let o_tx_control_usb1_clone = o_tx_control_usb1.clone();
            o_ws.on_upgrade(move |o_websocket| f_websocket_thread(
                o_websocket,
                o_rx_receiver,
                o_tx_sender_clone,
                o_tx_control_usb1_clone 
            ))
        })
        .or(warp::fs::dir("public"));

    let mut a_n_ip = [127,0,0,1];
    let s_hostname = gethostname();
    if(s_hostname == "raspi-desktop"){
        a_n_ip = [192,168,1,105];
    }
    tokio::spawn(
        warp::serve(routes)
        .tls()
        // RSA
        .cert_path("cert.pem")
        .key_path("key.pem")
        .run((a_n_ip, 3030))
    );
    println!("webserver running at http://127.0.0.1:3030/");
    println!("websocket running at ws://127.0.0.1:3030/ws");


    // 

    while let Ok(o) = o_rx_control_usb2.recv() {
        // println!("Received data: {:?}", data.a_n_u8_usb_read_result);
    
        if let Some(o_input_device) = o.v_o_input_device{

            o_tx_sender_clone.send(serde_json::to_string(
                &json!({
                    "o_input_device": o_input_device  // Embed the struct within the outer property
                })
            ).expect("failed to convert to json"));
        }

        // tx_clone.send(data.a_n_u8_usb_read_result.unwrap());
        // tx_clone.send(vec![12,2,2,3,]).unwrap();

    }

    // Send messages from the main loop or based on other events
    loop {
        if o_tx_sender_clone.send(String::from("1,2,3,4,5,6,7,8,9,10")).is_err() {
            eprintln!("Failed to send message to WebSocket clients");
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
    
}

