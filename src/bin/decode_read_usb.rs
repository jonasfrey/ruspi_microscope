use rusb::{DeviceHandle, GlobalContext, open_device_with_vid_pid, Direction, TransferType};
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
use std::io;
use std::io::Write;
use std::process::Command;
use crate::functions::f_update_o_input_device;
fn main() -> Result<(), Box<dyn std::error::Error>> {


    let output = Command::new("lsusb")
        .output()
         .expect("Failed to execute command");

    let output_str = String::from_utf8_lossy(&output.stdout);

    let a_s_line: Vec<_> = output_str
        .split('\n')
        .filter(|s| !s.trim().is_empty())
        .collect();

    let a_s_line2: Vec<_> = a_s_line
        .iter()
        .enumerate()
        .map(|(idx, line)| format!("{}:{}", idx + 1, line))
        .collect();

    println!("Select an input:");
    for s_line in &a_s_line2 {
        println!("{}", s_line);
    }

    let mut input = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let selected_index: usize = input.trim().parse::<usize>().expect("Please enter a number") - 1;

    let s_device = a_s_line[selected_index];
    let vid_pid = s_device.split_whitespace().nth(5).unwrap();
    let parts: Vec<_> = vid_pid.split(':').collect();
    let n_id_vendor = u16::from_str_radix(parts[0], 16).expect("Failed to parse VID");
    let n_id_product = u16::from_str_radix(parts[1], 16).expect("Failed to parse PID");

    println!("Selected VID: {:04x}, PID: {:04x}", n_id_vendor, n_id_product);

    // let s_device = "Bus 001 Device 004: ID 2563:0526 ShenZhen ShanWan Technology Co., Ltd. Android Gamepad";
    // let a_s_part: Vec<&str> = s_device.split_whitespace().collect();
    // let s_part__id = a_s_part[5]; // "2563:0526"
    // let a_s_id: Vec<&str> = s_part__id.split(':').collect();
    // Parsing the Vendor ID and Product ID from the substring
    // let n_id_vendor = u16::from_str_radix(a_s_id[0], 16).expect("Failed to parse vendor ID");
    // let n_id_product = u16::from_str_radix(a_s_id[1], 16).expect("Failed to parse product ID");

    // Assuming open_device_with_vid_pid returns a Result and we handle the error case
    let mut o_device_handle = open_device_with_vid_pid(n_id_vendor, n_id_product)
        .expect(&format!("Cannot open USB device {:?}, is the device connected? are you root?", s_device).to_string());
    
    
    let mut o_timeout = Duration::from_millis(1000);
    let mut n_len_a_n_u8__readout = 32;
    let mut n_interface = 0;
    let mut n_address_endpoint_in = 0x81;
    let config_desc = o_device_handle.device().active_config_descriptor().expect("Failed to get configuration descriptor");
    let mut b_no_in_interface_found_yet = true;
    for interface in config_desc.interfaces() {

        for interface_desc in interface.descriptors() {
            for endpoint_desc in interface_desc.endpoint_descriptors() {
                if endpoint_desc.direction() == Direction::In
                    && endpoint_desc.transfer_type() == TransferType::Interrupt
                {
                    if(b_no_in_interface_found_yet || interface_desc.class_code() == 3){

                        println!("Found an IN endpoint: 0x{:02x}", endpoint_desc.address());
                        n_address_endpoint_in = endpoint_desc.address();
                        let n = endpoint_desc.interval() as f32;
                        n_interface = interface.number();
                        o_timeout = Duration::from_millis((n*8.00) as u64);// account for overhead and ensure reliability
                        n_len_a_n_u8__readout = endpoint_desc.max_packet_size();
                    }
                }
            }
        }
    }
    println!("found interrupt interface: {}, endpoint IN addr:{}, timeout:{:?}, n_len_a_n_u8_readout {:?}", n_interface, n_address_endpoint_in, o_timeout, n_len_a_n_u8__readout);

    let _ = o_device_handle.set_auto_detach_kernel_driver(true).expect("cannot set auto a- de- tach of the kernel driver");
    println!(" kernel_driver_active {:?}", o_device_handle.kernel_driver_active(n_interface));
    // let _ = o_device_handle.detach_kernel_driver(n_interface).expect("cannot detach kernel driver");
    let _ = o_device_handle.claim_interface(n_interface).expect("Cannot claim interface");

    // std::process::exit(0);

    let mut a_n_u8__readout = vec![0u8; n_len_a_n_u8__readout.into()];

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

        match o_device_handle.read_interrupt(
            n_address_endpoint_in,
            &mut a_n_u8__readout,
            Duration::from_millis(10000),
            //  o_timeout
        ) {
            Ok(n_bytes_read) => {
                // a_n_u8__readout.truncate(n_bytes_read);
                println!("Read from USB device success, bytes read: {:?}", a_n_u8__readout);
                let mut n_c = 0;
                for n_u8 in &a_n_u8__readout{
                    n_c+=1;
                    if(n_c % 4 == 0){
                        print!("\n")
                    }
                    print!("{:#010b},", n_u8);
                }

                if let Some(ref mut o_input_device) = v_o_input_device{

                    f_update_o_input_device(
                        o_input_device, 
                        &a_n_u8__readout
                    );
                    for o_input_sensor in &o_input_device.a_o_input_sensor{

                        println!("{:?}:{:?} {:?}", o_input_sensor.s_name, o_input_sensor.n_nor, o_input_sensor.v_o_num_str_value);
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
