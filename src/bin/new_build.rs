use tokio::sync::broadcast;
use warp::ws::{Message, WebSocket};
use warp::Filter;
use futures::{StreamExt, SinkExt};
use std::time::Duration;
use std::thread;
use std::sync::{mpsc, Arc, Mutex};
use rusb::{DeviceHandle, GlobalContext, open_device_with_vid_pid};


enum ControlCommand {
    Start,
    Stop,
    SwitchDevice(u16, u16),  // Vendor ID, Product ID
}
struct SendData{
    a_n_u8_usb_read_result: Option<Vec<u8>>,
}

fn usb_read_thread(
    control_tx: mpsc::Sender<SendData>,
    control_rx: mpsc::Receiver<ControlCommand>, 
) {
    let mut o_device_handle_option: Option<DeviceHandle<GlobalContext>> = None;
    let mut b_usb_readout_active = false;
    let mut n_id_vendor = 0;
    let mut n_id_product = 0;
    let mut timeout = Duration::from_millis(1000);

    loop {
        // Check for control commands
        match control_rx.try_recv() {
            Ok(ControlCommand::Start) => {
                b_usb_readout_active = true;
            },
            Ok(ControlCommand::Stop) => {
                b_usb_readout_active = false;
            },
            Ok(ControlCommand::SwitchDevice(vid, pid)) => {

                println!("Changing from old vid:pid 0x{:02x}:0x{:02x} to vid:pid 0x{:02x}:0x{:02x}", n_id_vendor, n_id_product, vid, pid);
                if(o_device_handle_option.is_some()){
                    println!("release old iface and attach kernel driv");
                    if let Some(ref mut o_device_handle) = o_device_handle_option{
                        let _ = o_device_handle.release_interface(0).expect("cannot release interface");
                        // let _ = o_device_handle.attach_kernel_driver(0).expect("cannot attach kernel driver");
                        
                    }
                    // release the old interface and attach the kernel driver

                }
                n_id_vendor = vid;
                n_id_product = pid;
                o_device_handle_option = open_device_with_vid_pid(vid, pid);
                if let Some(ref mut o_device_handle ) = o_device_handle_option {
                    let n_idx_iface = 0;
                    let _ = o_device_handle.set_auto_detach_kernel_driver(true).expect("cannot set auto a- de- tach of the kernel driver");
                    let _ = o_device_handle.claim_interface(n_idx_iface).expect("Cannot claim interface");
                }

                b_usb_readout_active = o_device_handle_option.is_some();
            },
            Err(mpsc::TryRecvError::Empty) => {},
            Err(_) => break,  // Exit on other errors
        }

        if b_usb_readout_active {
            let mut buffer = vec![0u8; 32];
            if let Some(ref mut o_device_handle) = o_device_handle_option{

                match o_device_handle.read_interrupt(0x81, &mut buffer, timeout) {
                    Ok(n_bytes_read) => {
                        buffer.truncate(n_bytes_read);
                        println!("Read from USB device success, bytes read: {:?}", buffer);
                        control_tx.send(SendData{
                            a_n_u8_usb_read_result: Some(buffer)
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

async fn websocket_thread(
    mut o_websocket: WebSocket,
    mut o_rx_receiver: broadcast::Receiver<String>, 
    mut o_tx_sender_clone: broadcast::Sender<String>
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
            if let Ok(text) = result.to_str() {
                println!("websocket received message");
                if let Ok(data) = serde_json::from_str::<serde_json::Value>(text) {
                    if let Some(image_data) = data.get("s_data_url").and_then(|img| img.as_str()) {
                        // Decode the Base64 image data
                        let image_bytes = base64::decode(image_data.strip_prefix("data:image/jpeg;base64,").unwrap_or("")).unwrap();
                        // Save the decoded bytes to a file
                        let mut file = std::fs::File::create("output_image.jpeg").unwrap();
                        file.write_all(&image_bytes).unwrap();
                        println!("Image saved successfully.");
                        // Process the image bytes as needed
                        println!("image bytes {:?}",  image_bytes);
                    }
                    if let Some(s) = data.get("s_command_to_run").and_then(|s2| s2.as_str()) {
                        
                        let a_s_arg :Vec<&str> = s.split_whitespace().collect();
                        let a_s_command_allowed = ["lsusb", "touch lol_test"];
                        if !a_s_command_allowed.contains(&s) {
                            let sresp = format!("command '{}' not allowed, allowed are {:?}", s, a_s_command_allowed);
                            ws_tx.send(Message::text(sresp)).await.unwrap();
                        }else{
                            let a_s_arg2 = if a_s_arg.len() > 1 { &a_s_arg[1..] } else { &[] };
                            let s_out =Command::new(a_s_arg[0])
                                .args(a_s_arg2)
                                .output()
                                .expect("failed to execute process");
                            println!("s_out {:?}", s_out);
    
                            // Convert output to a string and send it back
                            let output_string = String::from_utf8_lossy(&s_out.stdout);
                            ws_tx.send(Message::text(output_string)).await.unwrap();
    
                                    // println!("message {:?}", sdev);
                            // i want to be able to send data here... 
                        }
                    }
                    if let Some(s) = data.get("s_name_function").and_then(|s2| s2.as_str()) {
                        
                        let n_id_vendor = data.get("n_id_vendor").unwrap().as_i64().expect("value has to be number");
                        let n_id_product = data.get("n_id_product").unwrap().as_i64().expect("value has to be number");
                        if(s == "switch_usb_device"){
                        control_tx.send(ControlCommand::Stop).unwrap();
                        control_tx.send(ControlCommand::SwitchDevice(
                            n_id_vendor as u16, 
                            n_id_product as u16, 
                        )).unwrap();
                        control_tx.send(ControlCommand::Start).unwrap();

                        }
                        // let a_n_id_vendor_n_id_product: Vec<&str> = s.split(':').collect();

                        // control_tx.send(ControlCommand::Stop).unwrap();
                        // control_tx.send(ControlCommand::SwitchDevice(
                        //     a_n_id_vendor_n_id_product[0], 
                        //     a_n_id_vendor_n_id_product[1], 
                        // )).unwrap();
                        // control_tx.send(ControlCommand::Start).unwrap();
                    }


                }
            }

    }

    // Make sure the forwarding task is properly cleaned up
    let _ = forward_task.await;
}

#[tokio::main]
async fn main() {
    let n_messages_at_a_time_in_the_channel_buffer_max = 10;
    let (o_tx_sender, _) = broadcast::channel::<String>(n_messages_at_a_time_in_the_channel_buffer_max);
    let o_tx_sender_clone = o_tx_sender.clone();
    let routes = warp::path("ws")
        .and(warp::ws())
        .map(move |o_ws: warp::ws::Ws| {
            let o_rx_receiver: broadcast::Receiver<String> = o_tx_sender.subscribe();
            let o_tx_sender_clone = o_tx_sender.clone();

            o_ws.on_upgrade(move |o_websocket| websocket_thread(
                o_websocket,
                o_rx_receiver,
                 o_tx_sender_clone
            ))
        })
        .or(warp::fs::dir("public"));

    tokio::spawn(warp::serve(routes).run(([127, 0, 0, 1], 3030)));

    // Send messages from the main loop or based on other events
    loop {
        if o_tx_sender_clone.send(String::from("1,2,3,4,5,6,7,8,9,10")).is_err() {
            eprintln!("Failed to send message to WebSocket clients");
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
