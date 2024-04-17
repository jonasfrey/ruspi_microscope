use std::fs::File;
use std::io::Write;
use tokio::sync::broadcast;
use warp::{Filter};
use futures::{FutureExt, StreamExt};
use std::time::{Duration, Instant};
use tokio::time::sleep;
use futures::sink::SinkExt;


async fn usb_loop(tx: broadcast::Sender<Vec<u8>>) {
    let mut context = rusb::Context::new().unwrap(); // ensure this can also fail gracefully
    loop {
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
                        let image_bytes = base64::decode(image_data.strip_prefix("data:image/jpeg;base64,").unwrap_or("")).unwrap();
                        // Save the decoded bytes to a file
                        let mut file = File::create("output_image.jpeg").unwrap();
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
