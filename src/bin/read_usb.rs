use rppal::gpio::Gpio;
use serde_json::{Value, json};
use std::{error::Error, os::unix::process, process::exit, sync::{mpsc,Arc, Mutex}, thread::{self, JoinHandle}, time::{Duration, Instant, SystemTime, UNIX_EPOCH}};
use rusb::{Device, UsbContext, DeviceHandle, open_device_with_vid_pid};
use core::task::Context;
use std::process::Command;
// use tokio::sync::{Mutex, mpsc};
// use tokio::sync::mpsc;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use futures::SinkExt;
use tokio_tungstenite::accept_async;
// use tokio::stream::StreamExt;
use tungstenite::protocol::Message;
// use futures_util::stream::stream::StreamExt;
use futures::stream::StreamExt;
use tokio::runtime::Runtime;
use warp::Filter;




fn f_start_usb_read_thread<C: UsbContext + 'static>(
    device_handle: Arc<Mutex<DeviceHandle<C>>>,
    timeout: Duration,
) -> mpsc::Receiver<Vec<u8>> {
    // Create a channel for communicating USB read results back to the main thread
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || loop {
        
        let mut buffer = vec![0u8; 32]; // Adjust buffer size as needed
        {
            let mut o_device_handle = device_handle.lock().unwrap();
            let n_idx_iface = 0;
            let _ = o_device_handle.detach_kernel_driver(n_idx_iface);
            let o = o_device_handle.claim_interface(n_idx_iface);
        }
        {
            // Lock the device handle for the duration of the USB operation
            let handle = device_handle.lock().unwrap();
            match handle.read_interrupt(0x81, &mut buffer, timeout) {
                Ok(bytes_read) => {
                    println!("read from usb device success");
                    buffer.truncate(bytes_read); // Adjust buffer size to actual bytes read
                    tx.send(buffer.clone()).expect("Failed to send data through channel");
                }
                Err(e) => {
                    eprintln!("USB read error: {:?}", e);
                    // Handle the error as needed (e.g., break the loop, retry, etc.)
                }
            }
        }

        // Optional: sleep or yield to prevent the thread from monopolizing CPU resources
        thread::sleep(Duration::from_millis(1)); // Adjust based on your requirements
    });

    rx
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {



    // println!("{:?}", o_input_sensor);
    let devices = rusb::devices()?;

    let mut a_n_u8_read = [0u8; 32];
    let o_duration__timeout = std::time::Duration::from_secs(1);
    // println!("device found {}", o_device);

        // Bus 001 Device 025: ID 046d:c31c Logitech, Inc. Keyboard K120
        let n_id_vendor = 0x046d;
        let n_id_product = 0xc31c; 

    let mut o_device_handle = open_device_with_vid_pid(n_id_vendor,n_id_product).expect(
        "cannot open usb device"
    );

    
    // Start the USB read thread
    let o_arc_mutex_o_device_handle = Arc::new(Mutex::new(o_device_handle));
    let timeout = Duration::from_millis(100);
    
    let usb_read_receiver = f_start_usb_read_thread(o_arc_mutex_o_device_handle, timeout);
    
    
    loop{

        match usb_read_receiver.try_recv() {
            Ok(a_n_u8_read) => {
                // Process the data received from the USB device
                // println!("Received USB data: {:?}", a_n_u8_read);
            
                println!("read usb !");

            }
            Err(mpsc::TryRecvError::Empty) => {
                // No data available yet; the main thread can perform other work
            }
            Err(e) => {
                // Handle other kinds of errors (e.g., the sender was disconnected)
                // eprintln!("Channel receive error: {:?}", e);
                break;
            }
        }



    }




    Ok(())
}
