mod classes;
mod functions;

use rusb::{DeviceHandle, GlobalContext, open_device_with_vid_pid, Direction, TransferType};
use std::ptr::null;
use classes::A_o_input_device;
use classes::A_o_name_synonym;
use crate::functions::f_b_bool_button_down;
use std::io;
use std::io::Write;
use std::process::Command;
use crate::functions::f_update_o_input_device;
use crate::functions::f_o_stepper_28BYJ_48;
use functions::f_o_input_sensor_from_s_name;
use std::time::Instant;
use crate::functions::f_check_mic_sec_delta_and_potentially_step;
use crate::functions::f_o_sender_tx_spawn_thread_with_event_listener_for_stepper;
use std::{
    fs,
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {




    // get the usb controller 
    // Bus 001 Device 003: ID 045e:028e Microsoft Corp. Xbox360 Controller esperenza usb 2.4 ghz controller
    let n_id_vendor = 0x045e;
    let n_id_product = 0x028e;
        
    let n_iface = 0;
    let n_endpiont_read_in_interrupt = 0x81;
    let n_millis_timeout = 8000;
    let n_len_a_n_u8__readout = 32;
    let mut a_n_u8__readout = vec![0u8; n_len_a_n_u8__readout as usize];


    let mut o_device_handle = open_device_with_vid_pid(n_id_vendor, n_id_product)
    .expect(&format!("Cannot open USB device, is the device connected? are you root?").to_string());
    let _ = o_device_handle.set_auto_detach_kernel_driver(true).expect("cannot set auto a- de- tach of the kernel driver");
    println!(" kernel_driver_active {:?}", o_device_handle.kernel_driver_active(n_iface));
    // let _ = o_device_handle.detach_kernel_driver(n_iface).expect("cannot detach kernel driver");
    let _ = o_device_handle.claim_interface(n_iface).expect("Cannot claim interface");


    // find device to decode 
    let s_json = fs::read_to_string("./o_info.json").expect("Unable to read file");
    // Deserialize the JSON data into a serde_json::Value
    let v: serde_json::Value = serde_json::from_str(&s_json)?;
    let mut a_o_input_device: A_o_input_device = serde_json::from_value(v.get("a_o_input_device").expect("json must have a_o_input_device").clone()).expect("cannot decode json");
    let mut a_o_name_synonym: A_o_name_synonym = serde_json::from_value(v.get("a_o_name_synonym").expect("json must have a_o_name_synonym").clone()).expect("cannot decode json");
    println!("a_o_input_device {:?}", a_o_input_device);
    let mut o_input_device = a_o_input_device.iter_mut().find(|o| o.n_id_vendor == n_id_vendor && o.n_id_product == n_id_product)
        .expect("cannot find json definition for input device, you have to decode its endpoint input bytes and add a definition in .json file");


    // prepare the stepper motors



        //raspi pinout pin layout 
    // |----------------------|----------------------|
    // |_   3v3 power         |_   5v power          |
    // |_   GPIO 2 (SDA)      |_   5v power          |
    // |_   GPIO 3 (SCL)      |_   Ground            |
    // |_   GPIO 4 (GPCLK0)   |_   GPIO 14 (TXD)     |
    // |_   Ground            |_   GPIO 15 (RXD)     |
    // |_   GPIO 17           |_   GPIO 18 (PCM_CLK) |
    // |_   GPIO 27           |_   Ground            |
    // |_   GPIO 22           |_   GPIO 23           |
    // |_   3v3 power         |_   GPIO 24           |
    // |_   GPIO 10 (MOSI)    |_   Ground            |
    // |_   GPIO 9 (MISO)     |_   GPIO 25           |
    // |_   GPIO 11 (SCLK)    |_   GPIO 8 (CEO)      |
    // |_   Ground            |_   GPIO 7 (CE1)      |
    // |_   GPIO 0 (ID_SD)    |_   GPIO 1 (ID_SD)    |
    // |_   GPIO 5            |_   Ground            |
    // |_   GPIO 6            |_   GPIO 12 (PWM0)    |
    // |_   GPIO 13 (PWM1)    |_   Ground            |
    // |_   GPIO 19 (PCM_FS)  |_   GPIO 16           |
    // |_   GPIO 26           |_   GPIO 20 (PCM_DIN) |
    // |_   Ground            |_   GPIO 21 (PCM_DOUT)|
    // |----------------------|----------------------|

    let mut o_sender_tx_stepper_28BYJ_48_x = f_o_sender_tx_spawn_thread_with_event_listener_for_stepper([2,3,4,17]);
    let mut o_sender_tx_stepper_28BYJ_48_y = f_o_sender_tx_spawn_thread_with_event_listener_for_stepper([27,22,10,9]);
    let mut o_sender_tx_stepper_28BYJ_48_z = f_o_sender_tx_spawn_thread_with_event_listener_for_stepper([11,0,5,6]);


    loop{
        match o_device_handle.read_interrupt(
            n_endpiont_read_in_interrupt,
            &mut a_n_u8__readout,
            Duration::from_millis(n_millis_timeout),
            //  o_timeout
        ) {
            Ok(n_bytes_read) => {
                // a_n_u8__readout.truncate(n_bytes_read);
                println!("Read from USB device success, bytes read: {:?}", a_n_u8__readout);
                let mut n_c = 0;
                for n_u8 in &a_n_u8__readout{
                    n_c+=1;
                    if(n_c % 8 == 0){
                        print!("\n")
                    }
                    print!("{:#010b},", n_u8);
                }
                f_update_o_input_device(
                    o_input_device, 
                    &a_n_u8__readout
                );

                // for o_input_sensor in &o_input_device.a_o_input_sensor{
                //     println!("{:?}:{:?} {:?}", o_input_sensor.s_name, o_input_sensor.n_nor, o_input_sensor.v_o_num_str_value);
                // }

                let o_left_stick_x_axis = f_o_input_sensor_from_s_name(&o_input_device, "left_stick_x_axis").unwrap();
                let o_left_stick_y_axis = f_o_input_sensor_from_s_name(&o_input_device, "left_stick_y_axis").unwrap();
                let o_right_stick_x_axis = f_o_input_sensor_from_s_name(&o_input_device, "right_stick_x_axis").unwrap();
                let o_right_stick_y_axis = f_o_input_sensor_from_s_name(&o_input_device, "right_stick_y_axis").unwrap();

                let mut n_l_x = (o_left_stick_y_axis.n_nor-0.5)*2.;
                let mut n_l_y = (o_left_stick_x_axis.n_nor-0.5)*2.;
                let mut n_r_x = (o_right_stick_x_axis.n_nor-0.5)*2.;
                let mut n_r_y = (o_right_stick_y_axis.n_nor-0.5)*2.;

                n_l_x = if(n_l_x.abs() > 0.05){n_l_x} else{0.0};
                n_l_y = if(n_l_y.abs() > 0.05){n_l_y} else{0.0};
                n_r_x = if(n_r_x.abs() > 0.05){n_r_x} else{0.0};
                n_r_y = if(n_r_y.abs() > 0.05){n_r_y} else{0.0};
                println!("n_r_x,n_r_y,n_l_x,n_l_y {},{},{},{}", n_r_x,n_r_y,n_l_x,n_l_y);

                // println!("micsec delta {}", n_micsec_delta);
                o_sender_tx_stepper_28BYJ_48_x.send(
                    json!({ 
                        "n_rpm_nor": n_r_x.abs(),
                        "b_direction": if(n_r_x>0.0){true}else{false}
                    }).to_string()
                ).unwrap();
                o_sender_tx_stepper_28BYJ_48_y.send(
                    json!({ 
                        "n_rpm_nor": n_r_y.abs(),
                        "b_direction": if(n_r_y>0.0){true}else{false}
                    }).to_string()
                ).unwrap();
                o_sender_tx_stepper_28BYJ_48_z.send(
                    json!({ 
                        "n_rpm_nor": n_l_y.abs(),
                        "b_direction": if(n_l_y>0.0){true}else{false}
                    }).to_string()
                ).unwrap();

            },
            Err(e) => {
                eprintln!("USB read error: {:?}", e);
            }
        }

    }

    // loop{

    //     let mut o_input_device = o_mutex_arc_o_input_device.lock().unwrap();

    //     let o_left_stick_x_axis = f_o_input_sensor_from_s_name(&o_input_device, "left_stick_x_axis").unwrap();
    //     let o_left_stick_y_axis = f_o_input_sensor_from_s_name(&o_input_device, "left_stick_y_axis").unwrap();
    //     let o_right_stick_x_axis = f_o_input_sensor_from_s_name(&o_input_device, "right_stick_x_axis").unwrap();
    //     let o_right_stick_y_axis = f_o_input_sensor_from_s_name(&o_input_device, "right_stick_y_axis").unwrap();

    //     let mut n_l_x = (o_left_stick_y_axis.n_nor-0.5)*2.;
    //     let mut n_l_y = (o_left_stick_x_axis.n_nor-0.5)*2.;
    //     let mut n_r_x = (o_right_stick_x_axis.n_nor-0.5)*2.;
    //     let mut n_r_y = (o_right_stick_y_axis.n_nor-0.5)*2.;

    //     n_l_x = if(n_l_x.abs() > 0.05){n_l_x} else{n_l_x};
    //     n_l_y = if(n_l_y.abs() > 0.05){n_l_y} else{n_l_y};
    //     n_r_x = if(n_r_x.abs() > 0.05){n_r_x} else{n_r_x};
    //     n_r_y = if(n_r_y.abs() > 0.05){n_r_y} else{n_r_y};
    //     // println!("n_r_x,n_r_y,n_l_x,n_l_y {},{},{},{}", n_r_x,n_r_y,n_l_x,n_l_y);

    //     let n_micsec_now = o_instant.elapsed().as_micros();
    //     let n_micsec_delta = (n_micsec_now - n_micsec_last) as f64;
    //     // println!("micsec delta {}", n_micsec_delta);
        

    //     o_stepper_28BYJ_48_x.b_direction = if(n_r_x>0.0){true}else{false}; 
    //     o_stepper_28BYJ_48_x.n_rpm_nor = n_r_x.abs();
    //     o_stepper_28BYJ_48_y.b_direction = if(n_r_y>0.0){true}else{false}; 
    //     o_stepper_28BYJ_48_y.n_rpm_nor = n_r_y.abs();
    //     o_stepper_28BYJ_48_z.b_direction = if(n_r_y>0.0){true}else{false}; 
    //     o_stepper_28BYJ_48_z.n_rpm_nor = n_r_y.abs();
                    

    //     f_check_mic_sec_delta_and_potentially_step(&mut o_stepper_28BYJ_48_x);
    //     f_check_mic_sec_delta_and_potentially_step(&mut o_stepper_28BYJ_48_y);
    //     f_check_mic_sec_delta_and_potentially_step(&mut o_stepper_28BYJ_48_z);

    //     let n_micsec_probe_diff = n_micsec_sleep_probe - n_micsec_delta;
    //     // println!("probe sleep {}", n_micsec_probe_diff);
    //     if(n_micsec_probe_diff > 0.){
    //         thread::sleep(Duration::from_micros(
    //             (n_micsec_probe_diff as u128).try_into().unwrap()
    //         ));   
    //     }
    //     n_micsec_last = n_micsec_now;

    // }

    Ok(())
}
