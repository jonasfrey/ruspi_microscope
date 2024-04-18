use rusb::{DeviceHandle, GlobalContext, open_device_with_vid_pid};
use tokio::net::unix::pipe::Receiver;
use std::ptr::null;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use futures::{FutureExt, StreamExt};
use std::fs::File;
use std::io::Write;
use tokio::sync::broadcast;
use warp::{Filter};
use warp::ws::{Message, WebSocket};
use tokio::time::sleep;
use futures::sink::SinkExt;
use std::process::Command;


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
    ws: warp::ws::WebSocket,
    tx: tokio::sync::broadcast::Sender<Vec<u8>>, 
    control_tx: mpsc::Sender<ControlCommand>, 
    mut ws_rx: tokio::sync::mpsc::Receiver<Message>
) {
    let (mut ws_tx, mut ws_rx) = ws.split();
    let mut rx = tx.subscribe();

    // // Clone ws_tx for use in the spawned task
    // let ws_tx_clone = ws_tx.clone();
    
    // tokio::spawn(async move {
    //     while let Ok(data) = rx.recv().await {
    //         if ws_tx.send(warp::ws::Message::binary(data)).await.is_err() {
    //             break; // Exit loop if error occurs
    //         }
    //     }
    // });
    while let Some(message) = ws_rx.recv().await {
        ws_tx.send(message).await.expect("Failed to send WebSocket message");
    }

    // Inside your handle_websocket function
    while let Some(result) = ws_rx.next().await {
        if let Ok(msg) = result {
            if msg.is_close() {
                break;
            }
            if let Ok(text) = msg.to_str() {
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
    }

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (control_tx, control_rx) = mpsc::channel();
    let (send_data_tx, send_data_rx) = mpsc::channel();
    // Start the USB read thread
    thread::spawn(move || {
        usb_read_thread(send_data_tx, control_rx);
    });

    // // Sending commands to the USB read thread
    // // Bus 001 Device 002: ID 046d:c31c Logitech, Inc. Keyboard K120
    // control_tx.send(ControlCommand::SwitchDevice(0x046d, 0xc31c)).unwrap();
    // control_tx.send(ControlCommand::Start).unwrap();

    // // Simulate operational period
    // thread::sleep(Duration::from_secs(10));
    // // Bus 003 Device 007: ID 045e:028e Microsoft Corp. Xbox360 Controller
    // control_tx.send(ControlCommand::Stop).unwrap();
    // control_tx.send(ControlCommand::SwitchDevice(0x045e, 0x028e)).unwrap();
    // control_tx.send(ControlCommand::Start).unwrap();
    // thread::sleep(Duration::from_secs(52));


    // let (send_data_tx, mut send_data_rx) = mpsc::channel();
    // let (broadcast_tx, _) = broadcast::channel(10);
    let (ws_tx, ws_rx) = tokio::sync::broadcast::channel::<Message>(32);
    let (tx, _) = tokio::sync::broadcast::channel(10);
    let tx_clone = tx.clone();  // Keep a clone to send messages from the main loop

    // Web server and WebSocket setup
    let routes = warp::path("ws").and(warp::ws()).map(move |ws: warp::ws::Ws| {
        let tx = tx.clone();
        let control_tx_clone = control_tx.clone();
        ws.on_upgrade(move |socket| websocket_thread(socket, tx.clone(), control_tx_clone, ws_rx))
    }).or(warp::fs::dir("public")); // Serve static files from 'public' directory

    println!("webserver running at http://127.0.0.1:3030/");
    println!("websocket running at ws://127.0.0.1:3030/ws");
    // Start server
    tokio::spawn(warp::serve(routes).run(([127, 0, 0, 1], 3030)));

    println!("started server");



    // Example of how to handle received data
    while let Ok(data) = send_data_rx.recv() {
        println!("Received data: {:?}", data.a_n_u8_usb_read_result);
    
        // tx_clone.send(data.a_n_u8_usb_read_result.unwrap());
        // tx_clone.send(vec![12,2,2,3,]).unwrap();

    }
    Ok(())


}


enum ControlCommand {
    Start,
    Stop,
    SwitchDevice(u16, u16),  // Vendor ID, Product ID
}
struct SendData{
    a_n_u8_usb_read_result: Option<Vec<u8>>,
}