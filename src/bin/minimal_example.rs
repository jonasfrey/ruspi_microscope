use std::fs::File;
use std::io::Write;
use tokio::sync::broadcast;
use warp::{Filter};
use futures::{FutureExt, StreamExt};
use std::time::{Duration, Instant};
use tokio::time::sleep;
use futures::sink::SinkExt;
<<<<<<< Updated upstream
use rusb::{Device, UsbContext, DeviceHandle, open_device_with_vid_pid};
=======
use std::time::{SystemTime, UNIX_EPOCH};

>>>>>>> Stashed changes

async fn usb_loop(tx: broadcast::Sender<Vec<u8>>) {

    // Bus 001 Device 006: ID Bus 001 Device 006: ID 046d:c31c Logitech, Inc. Keyboard K120:c31c Logitech, Inc. Keyboard K120
    let mut n_vid = 0x046d;
    let mut n_pid = 0xc31c;
    //Bus 003 Device 007: ID 045e:028e Microsoft Corp. Xbox360 Controller
    // n_vid = 0x045e;
    // n_pid = 0x028e;
    let mut o_device_handle = open_device_with_vid_pid(
        n_vid,// n_id_vendor, 
        n_pid// n_id_product
    ).expect("cannot open usb device, are you root?");
    let n_idx_iface = 0;
    let _ = o_device_handle.detach_kernel_driver(n_idx_iface).expect("cannot detach kernel diver, is the device used by another program?");
    let _ = o_device_handle.claim_interface(n_idx_iface).expect("cannot claim interface, is the device used by another program");
    // let mut context = rusb::Context::new().expect("cannot create new context, rusb::Context::new"); // ensure this can also fail gracefully
    loop {
        let mut buffer = vec![0u8; 32]; // Adjust buffer size as needed
        let timeout = Duration::from_millis(100);
        match o_device_handle.read_interrupt(0x81, &mut buffer, timeout) {
            Ok(bytes_read) => {

                println!("Read from USB device success, bytes read: {:?}", bytes_read);
                buffer.truncate(bytes_read); // Adjust buffer size to actual bytes read
                if tx.send(buffer.clone()).is_err() {
                    eprintln!("Failed to send data through channel");
                    break;
                }
            },
            Err(e) => {
                eprintln!("USB read error: {:?}", e);
                break;
            }
        }

        let data = vec![1, 2, 3]; // Dummy data simulating USB read
        if tx.send(data).is_err() {
            // println!("Warning: No subscribers available to receive data.");
        }
        sleep(Duration::from_micros(100)).await;
    }
}


async fn handle_websocket(ws: warp::ws::WebSocket, tx: broadcast::Sender<Vec<u8>>) {
    let (mut ws_tx, mut ws_rx) = ws.split();
    let mut rx = tx.subscribe();

    tokio::spawn(async move {
        while let Ok(data) = rx.recv().await {
            if ws_tx.send(warp::ws::Message::binary(data)).await.is_err() {
                break; // Exit loop if error occurs
            }
        }
    });

    // Inside your handle_websocket function
    while let Some(result) = ws_rx.next().await {
        if let Ok(msg) = result {
            if msg.is_close() {
                break;
            }
            if let Ok(text) = msg.to_str() {
                if let Ok(data) = serde_json::from_str::<serde_json::Value>(text) {
                    if let Some(image_data) = data.get("s_data_url").and_then(|img| img.as_str()) {
                        // Decode the Base64 image data
                        let image_bytes = base64::decode(image_data.strip_prefix("data:image/png;base64,").unwrap_or("")).unwrap();
                        // Save the decoded bytes to a file
                        let start = SystemTime::now();
                        let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
                        let unix_timestamp = since_the_epoch.as_secs(); // Seconds since the epoch
                    
                        // Create the file with the timestamp in the filename
                        let file_name = format!("image_{}.png", unix_timestamp);
                        let mut file = File::create(file_name).unwrap();

                        file.write_all(&image_bytes).unwrap();
                        println!("Image saved successfully.");
                        // Process the image bytes as needed
                        println!("image bytes {:?}",  image_bytes);
                    }
                }
            }
        }
    }

}

#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel(10);
    
    // USB reading loop
    let usb_task = tokio::spawn(usb_loop(tx.clone()));

    // Web server and WebSocket setup
    let routes = warp::path("ws").and(warp::ws()).map(move |ws: warp::ws::Ws| {
        let tx = tx.clone();
        ws.on_upgrade(move |socket| handle_websocket(socket, tx.clone()))
    }).or(warp::fs::dir("public")); // Serve static files from 'public' directory

    println!("webserver running at http://127.0.0.1:3030/");
    println!("websocket running at ws://127.0.0.1:3030/ws");
    // Start server
    tokio::spawn(warp::serve(routes).run(([127, 0, 0, 1], 3030)));

    // Main loop that sleeps every 100 microseconds
    let mut interval = tokio::time::interval(Duration::from_micros(100));
    loop {
        interval.tick().await;
    }
}
