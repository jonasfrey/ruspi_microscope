use rusb::{DeviceHandle, GlobalContext, open_device_with_vid_pid};
use std::ptr::null;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::fs;
use serde_json;
use classes::A_o_input_device;
use classes::A_o_name_synonym;
pub mod classes;
pub mod functions;
use crate::functions::f_b_bool_button_down;

use crate::functions::f_update_o_input_device;
fn main() -> Result<(), Box<dyn std::error::Error>> {

    let s_device = "Bus 001 Device 004: ID 2563:0526 ShenZhen ShanWan Technology Co., Ltd. Android Gamepad";

    let a_s_part: Vec<&str> = s_device.split_whitespace().collect();
    let s_part__id = a_s_part[5]; // "2563:0526"
    let a_s_id: Vec<&str> = s_part__id.split(':').collect();

    // Parsing the Vendor ID and Product ID from the substring
    let n_id_vendor = u16::from_str_radix(a_s_id[0], 16).expect("Failed to parse vendor ID");
    let n_id_product = u16::from_str_radix(a_s_id[1], 16).expect("Failed to parse product ID");

    // Assuming open_device_with_vid_pid returns a Result and we handle the error case
    let mut o_device_handle = open_device_with_vid_pid(n_id_vendor, n_id_product)
        .expect(&format!("Cannot open USB device {:?}, is the device connected? are you root?", s_device).to_string());
    
    
    let n_idx_iface = 0;
    let _ = o_device_handle.set_auto_detach_kernel_driver(true).expect("cannot set auto a- de- tach of the kernel driver");
    let _ = o_device_handle.claim_interface(n_idx_iface).expect("Cannot claim interface");

    let mut a_n_u8 = vec![0u8; 32];

    let o_timeout = Duration::from_millis(1000);



    let s_json = fs::read_to_string("./o_info.json").expect("Unable to read file");
    // Deserialize the JSON data into a serde_json::Value
    let v: serde_json::Value = serde_json::from_str(&s_json)?;



    let mut a_o_input_device: A_o_input_device = serde_json::from_value(v.get("a_o_input_device").expect("json must have a_o_input_device").clone()).expect("cannot decode json");
    let mut a_o_name_synonym: A_o_name_synonym = serde_json::from_value(v.get("a_o_name_synonym").expect("json must have a_o_name_synonym").clone()).expect("cannot decode json");
    println!("a_o_input_device {:?}", a_o_input_device);
    let mut v_o_input_device = a_o_input_device.iter_mut().find(|o| o.n_id_vendor == n_id_vendor && o.n_id_product == n_id_product);
    
    
    if(v_o_input_device.is_none()){
        println!("cannot find o_input_device and therefore the redout report from 0x81 cannot be decoded,
        you have to decode the output yourself, here you go in a loop of readouts, try to 
        press some buttons and watch for bits change, or try to see some bits grouped as 8, 16 or 32 bits, check if some bits
        change to 1111 1111 or to 0000 0000 when you push an axis of a acontroller etc.
        ")
    }

    
    loop{

        match o_device_handle.read_interrupt(0x81, &mut a_n_u8, o_timeout) {
            Ok(n_bytes_read) => {
                a_n_u8.truncate(n_bytes_read);
                println!("Read from USB device success, bytes read: {:?}", a_n_u8);
                for n_u8 in &a_n_u8{
                print!("{:#010b},", n_u8);
                }

                if let Some(ref mut o_input_device) = v_o_input_device{

                    f_update_o_input_device(
                        o_input_device, 
                        &a_n_u8
                    );
                    for o_input_sensor in &o_input_device.a_o_input_sensor{

                        // println!("{:?}:{:?} {:?}", o_input_sensor.s_name, o_input_sensor.n_nor, o_input_sensor.v_o_num_str_value);
                    }
                    // read a single value via its synonym name specific device name
                    
                    // 'A' on xobx, 
                    // 'cross' on playstation
                    // let n = f_n_nor(
                    //     o_input_device, 
                    //     &Some(a_o_name_synonym.clone()), 
                    //     "A"
                    // );
                    let b__dpad_down = f_b_bool_button_down(
                        o_input_device, 
                        &a_o_name_synonym, 
                        "dpad_down"
                    );
                    println!("dpad_down {}", b__dpad_down);
                    // o_input_device.a_o_input_sensor.iter(|o|)
                    // println!("{:?}", o_input_device);

                }


                // Send data to the main application if needed
            },
            Err(e) => {
                eprintln!("USB read error: {:?}", e);
            }
        }
        
    }
    // Use the device handle as needed
    println!("Device opened successfully!");

    Ok(())

    
}
