use tokio::sync::broadcast;
use warp::ws::{Message, WebSocket};
use warp::Filter;
use futures::{StreamExt, SinkExt};

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
        if let Ok(s_msg) = result {
            println!("Received message from WebSocket client: {:?}", s_msg);

            o_tx_sender_clone.send(String::from("message received, heres a new one sent!"));
            // can i send to the forward task here?
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
