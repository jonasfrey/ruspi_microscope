mod classes;
mod functions;

use std::{
    thread, 
    time::{
        Duration
    },
    sync::{
        mpsc
    }
};
use serde_json::{
    json, 
    Value
};

use classes::O_input_sensor_value;
use crate::functions::f_o_sender_tx_spawn_thread_with_event_listener_for_stepper;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {



    let o_instant = std::time::Instant::now();

    let o_sender_tx = f_o_sender_tx_spawn_thread_with_event_listener_for_stepper([2,3,4,17]);
    o_sender_tx.send(
        json!({ "n_rpm_nor": 0.2 }).to_string()
    ).unwrap();
    thread::sleep(Duration::from_millis(
        1000
    )); 
    o_sender_tx.send(
        json!({ "n_rpm_nor": 0.5 }).to_string()
    ).unwrap();
    thread::sleep(Duration::from_millis(
        1000
    ));   
    o_sender_tx.send(
        json!({ "n_rpm_nor": 0.0 }).to_string()
    ).unwrap();
    thread::sleep(Duration::from_millis(
        1000
    ));   
    o_sender_tx.send(
        json!({ "b_direction": true }).to_string()
    ).unwrap();
    thread::sleep(Duration::from_millis(
        1000
    ));  
    o_sender_tx.send(
        json!({ "n_rpm_nor": 0.04 }).to_string()
    ).unwrap();
    thread::sleep(Duration::from_millis(
        1000
    ));  
    o_sender_tx.send(
        json!({ "b_direction": false }).to_string()
    ).unwrap();
    thread::sleep(Duration::from_millis(
        1000
    ));  
    o_sender_tx.send(
        json!({ "n_rpm_nor": 0.2 }).to_string()
    ).unwrap();
    thread::sleep(Duration::from_millis(
        1000
    ));  
    o_sender_tx.send(
        json!({ "b_direction": true }).to_string()
    ).unwrap();
    thread::sleep(Duration::from_millis(
        1000
    ));  
    o_sender_tx.send(
        json!({ "n_rpm_nor": 0.2 }).to_string()
    ).unwrap();
    thread::sleep(Duration::from_millis(
        1000
    ));   
    loop{
        // println!("main idle loop");
    }

    Ok(())
}
