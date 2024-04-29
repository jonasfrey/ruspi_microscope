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
        ErrorKind,
        Write
    },
    fs::{
        self, 
        File
    },
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
    json, 
    Map, 
    Value
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
    f_b_gpio_available,
    f_update_o_input_device,
    f_o_sender_tx_spawn_thread_with_event_listener_for_stepper,
    f_o_input_sensor_from_s_name
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
    // println!("a_o_input_device {:?}", a_o_input_device);
    let mut v_o_input_device: Option<&mut O_input_device> = None;
    let mut o_device_handle_option: Option<DeviceHandle<GlobalContext>> = None;
    let mut b_usb_readout_active = false;
    let mut n_id_vendor = 0;
    let mut n_id_product = 0;
    let mut n_id_vendor_old = 0;
    let mut n_id_product_old = 0;
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

                        let o_command_old = Command::new("lsusb")
                                            .args(["-d", &format!("{:#02x}:{:#02x}", n_id_vendor_old, n_id_product_old)])
                                            .output()
                                            .expect("failed to execute process");
                        let o_command_new = Command::new("lsusb")
                                        .args(["-d", &format!("{:#02x}:{:#02x}", n_id_vendor, n_id_product)])
                                        .output()
                                        .expect("failed to execute process");
                        n_id_vendor_old = vid;
                        n_id_product_old = pid;
                        println!(
                            "switching usb device: 
                            old: {:?}
                            new: {}
                        ",
                        String::from_utf8_lossy(&o_command_old.stdout),
                        String::from_utf8_lossy(&o_command_new.stdout)
                        );
                        let o_config_descriptor = o_device_handle.device().active_config_descriptor().expect("Failed to get configuration descriptor");
                        let mut b_interface_found: bool = false;

                        for o_interface in o_config_descriptor.interfaces() {
                            for o_interface_descriptor in o_interface.descriptors() {
                                for o_endpoint_descriptor in o_interface_descriptor.endpoint_descriptors() {
                                    let n_dir = o_endpoint_descriptor.direction();
                                    let n_tt = o_endpoint_descriptor.transfer_type();
                                    let n_class = o_interface_descriptor.class_code();
                                    let n_ifacenum = o_interface.number();
                                    println!(
                                        "n_dir:{:?}
                                        n_tt:{:?}
                                        n_ifacenum:{:?}
                                        n_class:{:?}",
                                        n_dir,
                                        n_tt,
                                        n_ifacenum, 
                                        n_class
                                    );
                                    if n_dir == Direction::In
                                        && n_tt == TransferType::Interrupt
                                    {
                
                                        n_address_endpoint_in = o_endpoint_descriptor.address();
                                        let n = o_endpoint_descriptor.interval() as f32;
                                        n_interface = o_interface.number();
                                        o_timeout = Duration::from_millis((n*5.00) as u64);// account for overhead and ensure reliability
                                        n_len_a_n_u8__readout = o_endpoint_descriptor.max_packet_size();
                                        if(
                                            n_class == n_class_code_human_interface_device
                                        ){
                                            b_interface_found = true;
                                        }
                                        // some devices have 255 which is 0xff usb class code FFh Both Vendor Specific
                                            
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
                            let _ = o_device_handle.claim_interface(n_interface).expect(
                                &format!("Cannot claim interface, {}", n_interface)
                            );
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
                        
                            // for o_input_sensor in &o_input_device.a_o_input_sensor{

                            //     println!("{:?}:{:?} {:?}", o_input_sensor.s_name, o_input_sensor.n_nor, o_input_sensor.v_o_num_str_value);
                            // }
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

const s_path_file_abs__config: &'static str = "./o_config.json";

fn f_b_write_s_json_o_config(s_str: &str) -> bool{
    let mut file = File::create(s_path_file_abs__config).expect("Failed to create file");

    file.write_all(s_str.as_bytes()).expect("Failed to write to file");
    return true;
    // println!("Data written to {}", s_path_file_abs__config);
}

fn f_s_json_o_config() -> String {
    match fs::read_to_string(s_path_file_abs__config) {
        Ok(s_content) => s_content,
        Err(e) => {
            if let ErrorKind::NotFound = e.kind() {
                "{}".to_string() // Return an empty string if the file is not found
            } else {
                panic!("Failed to read file: {:?}", e) // Panic for other errors
            }
        }
    }
}

async fn f_websocket_thread(
    mut o_websocket: WebSocket,
    mut o_rx_receiver: broadcast::Receiver<String>, 
    mut o_tx_sender_clone: broadcast::Sender<String>, 
    o_tx_control_usb: mpsc::Sender<ControlCommand>, 
    v_o_sender_tx_stepper_28BYJ_48_x:Option<mpsc::Sender::<String>>,
    v_o_sender_tx_stepper_28BYJ_48_y:Option<mpsc::Sender::<String>>,
    v_o_sender_tx_stepper_28BYJ_48_z: Option<mpsc::Sender::<String>>
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

                    // s_name_function must be present as it is an identifier of what to do on the server
                    if let Some(s_name_function) = v_json_parsed.get("s_name_function").and_then(|v: &serde_json::Value| v.as_str()) {
                        // if 's_uuid' is present the response json should also contain it since the client expects it as a response
                        if let Some(s_uuid) = v_json_parsed.get("s_uuid").and_then(|v: &serde_json::Value| v.as_str()) {
                            let mut o_response = Map::new();                                
                                
                            if(s_name_function == "hello"){
                                o_response.insert("s_res".to_string(), json!("World !"));
                                o_response.insert("o_child".to_string(), json!({ "s_stdout__lsusb": "yes" }));
                            }

                            if(s_name_function == "f_control_stepper_motor"){

                                if let Some(s_axis) = v_json_parsed.get("s_axis").and_then(|v: &serde_json::Value| v.as_str()) {

                                    println!("trying to control stepper motor: {:?}", v_json_parsed);
                                    if(s_axis == "x"){
                                        if let Some(ref o) = v_o_sender_tx_stepper_28BYJ_48_x{
                                            o.send(
                                                serde_json::from_value(v_json_parsed.clone()).expect("invalid serde_json value")
                                            ).unwrap();
                                        }
                                    }
                                    if(s_axis == "y"){
                                        if let Some(ref o) = v_o_sender_tx_stepper_28BYJ_48_y{
                                            o.send(
                                                serde_json::from_value(v_json_parsed.clone()).expect("invalid serde_json value")
                                            ).unwrap();
                                        }
                                    }
                                    if(s_axis == "z"){

                                        if let Some(ref o) = v_o_sender_tx_stepper_28BYJ_48_z{
                                            o.send(
                                                serde_json::from_value(v_json_parsed.clone()).expect("invalid serde_json value")
                                            ).unwrap();
                                        }
                                    }


                                }else{
                                    o_response.insert("s_error".to_string(), json!("the property 's_axis' must be included"));

                                }
    
    
                            }

                            if(s_name_function == "f_s_json_o_config"){
                                o_response.insert("s_json_o_config".to_string(), json!(f_s_json_o_config()));
                            }

                            if(s_name_function == "f_b_write_s_json_o_config"){
                                if let Some(s_json) = v_json_parsed.get("s_json_o_config").and_then(|v: &serde_json::Value| v.as_str()) {
                                    f_b_write_s_json_o_config(s_json);
                                    o_response.insert("b".to_string(), json!(true));
                                }
                            }

                            if(s_name_function == "f_o_command"){
                                if let Some(s_command) = v_json_parsed.get("s_command").and_then(|v: &serde_json::Value| v.as_str()) {

                                    let a_s_arg :Vec<&str> = s_command.split_whitespace().collect();
                                    let a_s_command_allowed = ["lsusb", "touch lol_test"];
                                    if !a_s_command_allowed.contains(&s_command) {
                                        let sresp = format!("command '{}' not allowed, allowed are {:?}", s_command, a_s_command_allowed);
                                        o_tx_sender_clone.send(sresp);
                                    }else{
                                        let a_s_arg2 = if a_s_arg.len() > 1 { &a_s_arg[1..] } else { &[] };
                                        let o_command = Command::new(a_s_arg[0])
                                            .args(a_s_arg2)
                                            .output()
                                            .expect("failed to execute process");
                                        // println!("s_out {:?}", o_command);
                
                                        // Convert output to a string and send it back
                                        let s_stdout = String::from_utf8_lossy(&o_command.stdout);
                                        let s_stderr = String::from_utf8_lossy(&o_command.stderr);
                                        let o_status = o_command.status;
                                        o_response.insert("n_return_code".to_string(), json!(o_status.code()));
                                        o_response.insert("s_stdout".to_string(), json!(s_stdout));
                                        o_response.insert("s_stderr".to_string(), json!(s_stderr));

                                        // println!("message {:?}", sdev);
                                        // i want to be able to send data here... 
                                    }
                                }
                            }

                            
                            o_response.insert("s_uuid".to_string(), json!(s_uuid));
                            o_tx_sender_clone.send(
                                serde_json::to_string(&Value::Object(o_response)).unwrap()
                            );
                        }
                        
                        println!("received s_name_function {}", s_name_function);
                        o_tx_sender_clone.send(String::from("message received, heres a new one sent!"));



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
        //raspi pinout pin layout 
    // |----------------------|----------------------|
    // |_   3v3 power         |_   5v power          |
    // |_   GPIO 2 (SDA)      |_   5v power          |
    // |_   GPIO 3 (SCL)      |_   Ground            |
    // |_   GPIO 4 (GPCLK0)   |_   GPIO 14 (TXD)     |
    // |_   Ground            |_   GPIO 15 (RXD)     |
    // |_   GPIO 17           |_   GPIO 18 (PCM_CLK) |
    // |_   GPIO 27           |_   Ground            |
    // |_   GPIO 22           |_   GPIO 23           |
    // |_   3v3 power         |_   GPIO 24           |
    // |_   GPIO 10 (MOSI)    |_   Ground            |
    // |_   GPIO 9 (MISO)     |_   GPIO 25           |
    // |_   GPIO 11 (SCLK)    |_   GPIO 8 (CEO)      |
    // |_   Ground            |_   GPIO 7 (CE1)      |
    // |_   GPIO 0 (ID_SD)    |_   GPIO 1 (ID_SD)    |
    // |_   GPIO 5            |_   Ground            |
    // |_   GPIO 6            |_   GPIO 12 (PWM0)    |
    // |_   GPIO 13 (PWM1)    |_   Ground            |
    // |_   GPIO 19 (PCM_FS)  |_   GPIO 16           |
    // |_   GPIO 26           |_   GPIO 20 (PCM_DIN) |
    // |_   Ground            |_   GPIO 21 (PCM_DOUT)|
    // |----------------------|----------------------|


    let b_gpio_available = f_b_gpio_available();
    
    let mut v_o_sender_tx_stepper_28BYJ_48_x = None;
    let mut v_o_sender_tx_stepper_28BYJ_48_y = None;
    let mut v_o_sender_tx_stepper_28BYJ_48_z = None;

    if(b_gpio_available){
        let a_n_pin_x = [2,3,4,17];
        let a_n_pin_y = [27,22,10,9];
        let a_n_pin_z = [11,0,5,6];
        v_o_sender_tx_stepper_28BYJ_48_x = Some(f_o_sender_tx_spawn_thread_with_event_listener_for_stepper(a_n_pin_x));
        v_o_sender_tx_stepper_28BYJ_48_y = Some(f_o_sender_tx_spawn_thread_with_event_listener_for_stepper(a_n_pin_y));
        v_o_sender_tx_stepper_28BYJ_48_z = Some(f_o_sender_tx_spawn_thread_with_event_listener_for_stepper(a_n_pin_z));
        println!("you are using a raspberry pi, make sure the stepper motors are wired like this");
        println!("x-axis stepper gpio pins: {:?}", a_n_pin_x);
        println!("y-axis stepper gpio pins: {:?}", a_n_pin_y);
        println!("z-axis stepper gpio pins: {:?}", a_n_pin_z);
    }else{
        println!("with some 3d printed hardware and byj28 stepper motors you can run this software to control the microscope with a usb controller")
    }

    let mut v_o_sender_tx_stepper_28BYJ_48_x__clone_for_main = v_o_sender_tx_stepper_28BYJ_48_x.clone();
    let mut v_o_sender_tx_stepper_28BYJ_48_y__clone_for_main = v_o_sender_tx_stepper_28BYJ_48_y.clone();
    let mut v_o_sender_tx_stepper_28BYJ_48_z__clone_for_main = v_o_sender_tx_stepper_28BYJ_48_z.clone();


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
            let mut v_o_sender_tx_stepper_28BYJ_48_x__clone_for_websocket = v_o_sender_tx_stepper_28BYJ_48_x.clone();
            let mut v_o_sender_tx_stepper_28BYJ_48_y__clone_for_websocket = v_o_sender_tx_stepper_28BYJ_48_y.clone();
            let mut v_o_sender_tx_stepper_28BYJ_48_z__clone_for_websocket = v_o_sender_tx_stepper_28BYJ_48_z.clone();
            o_ws.on_upgrade(move |o_websocket| f_websocket_thread(
                o_websocket,
                o_rx_receiver,
                o_tx_sender_clone,
                o_tx_control_usb1_clone,
                v_o_sender_tx_stepper_28BYJ_48_x__clone_for_websocket,
                v_o_sender_tx_stepper_28BYJ_48_y__clone_for_websocket,
                v_o_sender_tx_stepper_28BYJ_48_z__clone_for_websocket
            ))
        })
        .or(warp::fs::dir("public"));
    //use o_sender_tx_stepper_28BYJ_48_y here later
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



    while let Ok(o) = o_rx_control_usb2.recv() {
        // println!("Received data: {:?}", data.a_n_u8_usb_read_result);
    
        if let Some(o_input_device) = o.v_o_input_device{

            o_tx_sender_clone.send(serde_json::to_string(
                &json!({
                    "o_input_device": o_input_device  // Embed the struct within the outer property
                })
            ).expect("failed to convert to json"));
            println!("update");
            let o_left_stick_x_axis = f_o_input_sensor_from_s_name(&o_input_device, "left_stick_x_axis").unwrap();
            let o_left_stick_y_axis = f_o_input_sensor_from_s_name(&o_input_device, "left_stick_y_axis").unwrap();
            let o_right_stick_x_axis = f_o_input_sensor_from_s_name(&o_input_device, "right_stick_x_axis").unwrap();
            let o_right_stick_y_axis = f_o_input_sensor_from_s_name(&o_input_device, "right_stick_y_axis").unwrap();
    
            let mut n_l_x = (o_left_stick_y_axis.n_nor-0.5)*2.;
            let mut n_l_y = (o_left_stick_x_axis.n_nor-0.5)*2.;
            let mut n_r_x = (o_right_stick_x_axis.n_nor-0.5)*2.;
            let mut n_r_y = (o_right_stick_y_axis.n_nor-0.5)*2.;
    
    
            n_l_x = if(n_l_x.abs() > 0.05){n_l_x*0.5} else{0.0};
            n_l_y = if(n_l_y.abs() > 0.05){n_l_y*0.5} else{0.0};
            n_r_x = if(n_r_x.abs() > 0.05){n_r_x*0.5} else{0.0};
            n_r_y = if(n_r_y.abs() > 0.05){n_r_y*0.5} else{0.0};
            println!("n_r_x,n_r_y,n_l_x,n_l_y {},{},{},{}", n_r_x,n_r_y,n_l_x,n_l_y);


            if let Some(ref o) = v_o_sender_tx_stepper_28BYJ_48_x__clone_for_main{
                o.send(
                    json!({ 
                        "n_rpm_nor": n_r_x.abs(),
                        "b_direction": if(n_r_x>0.0){true}else{false}
                    }).to_string()
                ).unwrap();
            }
            if let Some(ref o) = v_o_sender_tx_stepper_28BYJ_48_y__clone_for_main{
                o.send(
                    json!({ 
                        "n_rpm_nor": n_r_y.abs(),
                        "b_direction": if(n_r_y>0.0){true}else{false}
                    }).to_string()
                ).unwrap();
            }
            if let Some(ref o) = v_o_sender_tx_stepper_28BYJ_48_z__clone_for_main{
                o.send(
                    json!({ 
                        "n_rpm_nor": n_l_y.abs(),
                        "b_direction": if(n_l_y>0.0){true}else{false}
                    }).to_string()
                ).unwrap();
            }
        }

        // tx_clone.send(data.a_n_u8_usb_read_result.unwrap());
        // tx_clone.send(vec![12,2,2,3,]).unwrap();

    }

    
}

