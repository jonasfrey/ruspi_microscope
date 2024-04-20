use rusb::{DeviceHandle, GlobalContext, open_device_with_vid_pid};
use std::ptr::null;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (control_tx, control_rx) = mpsc::channel();
    let (send_data_tx, send_data_rx) = mpsc::channel();
    // Start the USB read thread
    thread::spawn(move || {
        usb_read_thread(send_data_tx, control_rx);
    });

    // Sending commands to the USB read thread
    // Bus 001 Device 002: ID 046d:c31c Logitech, Inc. Keyboard K120
    control_tx.send(ControlCommand::SwitchDevice(0x046d, 0xc31c)).unwrap();
    control_tx.send(ControlCommand::Start).unwrap();

    // Simulate operational period
    thread::sleep(Duration::from_secs(10));
    // Bus 003 Device 007: ID 045e:028e Microsoft Corp. Xbox360 Controller
    control_tx.send(ControlCommand::Stop).unwrap();
    control_tx.send(ControlCommand::SwitchDevice(0x045e, 0x028e)).unwrap();
    control_tx.send(ControlCommand::Start).unwrap();
    thread::sleep(Duration::from_secs(52));

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