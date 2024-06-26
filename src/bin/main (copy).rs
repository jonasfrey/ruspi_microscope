use classes::O_input_sensor_value;
use rppal::gpio::Gpio;
use std::{error::Error, os::unix::process, process::exit, sync::{Arc, Mutex}, thread::{self, JoinHandle}, time::{Duration, Instant, SystemTime, UNIX_EPOCH}};
use rusb::{Device, UsbContext, open_device_with_vid_pid};
use core::task::Context;

use crate::classes::{O_input_device, O_stepper_28BYJ_48};

pub mod classes; 


pub mod runtimedata; 
use runtimedata::f_a_o_input_device;

fn f_n_from_string(s: &str) -> u32 {
    s.replace(|c: char| !c.is_digit(10), "").parse::<u32>().unwrap_or(0)
}

fn f_s_first_letter_uppercase(s: &str) -> String {
    s.char_indices()
        .nth(0)
        .map(|(i, c)| c.to_uppercase().collect::<String>() + &s[i + 1..])
        .unwrap_or_else(|| s.to_owned())
}

fn f_n_u64_from_params(
    a_n_u8: &[u8],
    n_idx_bit_start: usize,
    n_idx_bit_end: usize
) -> u64 {
    assert!(n_idx_bit_end >= n_idx_bit_start, "End bit must be greater or equal to start bit");
    let bit_length = n_idx_bit_end - n_idx_bit_start + 1;

    let mut n_res: u64 = 0;

    // Loop through each bit in the range
    for bit_index in n_idx_bit_start..=n_idx_bit_end {
        let n_idx_byte = bit_index / 8;
        let bit_position_in_byte: usize = (bit_index % 8);

        // Extract the bit from the byte
        let bit = (a_n_u8[n_idx_byte] >> bit_position_in_byte) & 1;

        // Shift the extracted bit to its position in the result
        n_res |= (bit as u64) << (n_idx_bit_end - bit_index);
    }
    n_res
}

fn f_convert_endianess(n_res: u64, bit_length: usize) -> u64 {
    match bit_length {
        16 => u16::from_be_bytes(n_res.to_be_bytes()[6..8].try_into().unwrap()) as u64,
        32 => u32::from_be_bytes(n_res.to_be_bytes()[4..8].try_into().unwrap()) as u64,
        64 => u64::from_be_bytes(n_res.to_be_bytes()),
        _ => n_res, // No conversion needed for bit lengths that don't match multi-byte data types
    }
}

fn f_n_value(
    a_n_u8: &[u8],
    n_idx_bit_start: usize,
    n_bits: usize,
    b_signed: bool,
    // b_float: bool,
    b_big_endian: bool
) -> u64 {
    // assert!(!b_float, "Floating point not supported directly."); // Simplification for this example

    let mut n_res: u64 = 0;
    let mut bit_count = 0;

    while bit_count < n_bits {
        let byte_index = (n_idx_bit_start + bit_count) / 8;
        let bit_index = (n_idx_bit_start + bit_count) % 8;
        let bit = if b_big_endian {
            (a_n_u8[byte_index] >> (7 - bit_index)) & 1
        } else {
            (a_n_u8[byte_index] >> bit_index) & 1
        };

        n_res |= (bit as u64) << bit_count;
        bit_count += 1;
    }

    // For non-floating points and assuming n_bits <= 64
    if b_signed && n_bits < 64 {
        // Sign extend if the highest bit of the result is set (negative number)
        let sign_bit = 1 << (n_bits - 1);
        if n_res & sign_bit != 0 {
            let mask = !((1 << n_bits) - 1);
            n_res |= mask; // Apply sign extension
        }

        (n_res as u64) // Assuming the caller knows to interpret this based on n_bits
    } else {
        n_res as u64 // Positive or unsigned
    }
}

// fn f_a_n_u8__from_params(
//     a_n_u8: &[u8],
//     n_idx_bit_start: usize,
//     n_bits: usize, 
//     b_aligned_left: bool
// ) -> u64 {

//     // input [ 0b1011_0111, 0b0101_1000, 0b1110_1110]
//     // n_idx_bit_start = 3
//     // n_bits = 11
//     // b_aligned_left = true
//     // expected output [ 0b1_0111010, ]
// }

fn f_update_o_input_device(
    o_input_device: &mut O_input_device,
    a_n_u8:  &[u8]//Vec<u8>
) {
    let mut n_idx_bit_start: usize = 0;
    let mut n_idx_bit_end: usize = 0;
    for o in &mut o_input_device.a_o_input_sensor {
        let n_bits = f_n_from_string(&o.s_type) as usize;
        let b_unsigned = o.s_type.contains('u');
        let b_signed = o.s_type.contains('i');
        let b_float = o.s_type.contains('f');
        // println!("bits {}", n_bits);
        n_idx_bit_end = (n_idx_bit_start + n_bits-1);
        // let mut n_res = f_n_u64_from_params(a_n_u8, n_idx_bit_start, n_idx_bit_end);
        let mut n_res = f_n_value(
            a_n_u8,
            n_idx_bit_start, 
            n_bits, 
            b_signed, 
            false
        );
        let n_bits_rounded_up = ((n_bits / 8) as f32).ceil() * 8.0;
        n_res = f_convert_endianess(n_res, n_bits_rounded_up as usize);
        // println!("bit index start:end {}:{}", n_idx_bit_start, n_idx_bit_end);
        let mut n_value_max = (1 << n_bits) - 1;

        if o.s_type.contains('i') {
            n_value_max = n_value_max / 2;
            // Handle signed integers if needed
        }
        if(n_bits <= 8){
            // skipping because f8 not existing in this case
            // if(b_float){
            //     o.o_input_sensor_value = Some(O_input_sensor_value::F8(n_res as f8))
            // }
            if(b_signed){
                o.o_input_sensor_value = Some(O_input_sensor_value::I8(n_res as i8));
                o.n_nor = (n_res as i8) as f64 / n_value_max as f64;
            }
            if(b_unsigned){
                o.o_input_sensor_value = Some(O_input_sensor_value::U8(n_res as u8));
                o.n_nor = (n_res as u8) as f64 / n_value_max as f64;

            }
        }
        if(n_bits > 8 && n_bits <= 16){
            // skipping because f16 not existing in this case
            // if(b_float){
            //     o.o_input_sensor_value = Some(O_input_sensor_value::f16(n_res as f16))
            // }
            if(b_signed){
                o.o_input_sensor_value = Some(O_input_sensor_value::I16(n_res as i16));
                o.n_nor = (n_res as i16) as f64 / n_value_max as f64;

            }
            if(b_unsigned){
                o.o_input_sensor_value = Some(O_input_sensor_value::U16(n_res as u16));
                o.n_nor = (n_res as u16) as f64 / n_value_max as f64;

            }
        }
        if(n_bits > 16 && n_bits <= 32){
            if(b_float){
                o.o_input_sensor_value = Some(O_input_sensor_value::F32(n_res as f32));
                o.n_nor = (n_res as f32) as f64 / n_value_max as f64;
            }
            if(b_signed){
                o.o_input_sensor_value = Some(O_input_sensor_value::I32(n_res as i32));
                o.n_nor = (n_res as i32) as f64 / n_value_max as f64;
            }
            if(b_unsigned){
                o.o_input_sensor_value = Some(O_input_sensor_value::U32(n_res as u32));
                o.n_nor = (n_res as u32) as f64 / n_value_max as f64;
            }
        }
        if(n_bits > 32 && n_bits <= 64){
            if(b_float){
                o.o_input_sensor_value = Some(O_input_sensor_value::F64(n_res as f64));
                o.n_nor = (n_res as f64) as f64 / n_value_max as f64;
            }
            if(b_signed){
                o.o_input_sensor_value = Some(O_input_sensor_value::I64(n_res as i64));
                o.n_nor = (n_res as i64) as f64 / n_value_max as f64;
            }
            if(b_unsigned){
                o.o_input_sensor_value = Some(O_input_sensor_value::U64(n_res as u64));
                o.n_nor = (n_res as u64) as f64 / n_value_max as f64;
            }
        }


        if let Some(ref a_o_enum_value) = o.a_o_num_str_value {
            o.o_num_str_value = a_o_enum_value.iter().find(|&e| e.n == n_res).cloned();
        }
        // println!("n_res {:#032b}", n_res);
        // println!("{} max {} o.o_input_sensor_value {:?} {}", o.s_name, n_value_max, o.o_input_sensor_value, o.n_nor);
        // println!("{: >40} {}", o.s_name, o.n_nor);

        n_idx_bit_start += n_bits as usize;
    }
}

// fn f_update_o_input_device(
//     o_input_device: &mut O_input_device,
//     a_n_u8:  &[u8]//Vec<u8>
// ) -> bool {

//     let mut n_bit = 0;
//             for o in &mut o_input_device.a_o_input_sensor {
//         let n_idx_byte = n_bit / 8; // e.g., 2
//         let n_bits = f_n_from_string(o.s_type); // e.g., 4 for 'u4'
//         let mut b_signed = false;
//         if o.s_type.contains('i'){
//             b_signed = true
//         } 

//         let n_idx_bit = n_bit % 8; // e.g., 4
//         let n_value_max = 2u64.pow(n_bits.try_into().unwrap()) - 1; // e.g., 2^4-1 = 15

//         let mut n_value_number = n_value_number;
//         if ![8, 16, 32, 64].contains(&n_bits.try_into().unwrap()) {
//             n_value_number = n_value_number >> n_idx_bit & n_value_max;
//         }
//         if o.s_type.contains('i') {
//             n_value_max /= 2;
//         }

//         o.value = Some(n_value_number.try_into().unwrap());
//         o.n_nor = n_value_number as f64 / n_value_max as f64;
//         if let Some(ref a_o_num_str_value) = o.a_o_num_str_value {
//             o.o_num_str_value = a_o_num_str_value.iter().find(|o_enum| o_enum.n == n_value_number);
//         }

//         n_bit += n_bits;
//         let v = if let Some(ref o_num_sta_o_num_str_value) = o.o_num_str_value {
//             &o_num_sta_o_num_str_value.s
//         } else {
//             &o.n_nor
//         };
//         println!("{:30}: {:?}", o.s_name, v);
//     }
//     return true
// }


fn f_update_o_stepper(
    o_stepper: &mut O_stepper_28BYJ_48
){

    // Lock the mutex to safely access the shared state
    let n_rpm = o_stepper.n_rpm_nor.abs() * o_stepper.n_rpm_max;
    let n_fullsteps_per_minute = o_stepper.n_fullsteps_per_round as f64 * n_rpm; 
    o_stepper.n_micsec_sleep_between_fullstep = (60*1000*1000) as f64 / n_fullsteps_per_minute;
    let n_micsec_between_substep = (o_stepper.n_micsec_sleep_between_fullstep as f64) / o_stepper.n_substeps_per_step as f64;
    let n_micsec_ts_now = o_stepper.o_instant.elapsed().as_micros();
    // println!("micsec elapsed {}", n_micsec_ts_now);
    // println!("micsec delta {}", n_micsec_ts_now - o_stepper.n_micsec_ts_last_step);
    if((n_micsec_ts_now - o_stepper.n_micsec_ts_last_step) > n_micsec_between_substep as u128){
        // next sub step
        o_stepper.n_micsec_ts_last_step = n_micsec_ts_now;
        o_stepper.n_substeps+=1;
        let n_dir = if(o_stepper.b_direction){ 1 }else{ -1};
        let mut n_idx_a_o_pin = o_stepper.n_idx_a_o_pin as i32 + n_dir as i32;
        let n_len = o_stepper.a_o_pin.len() as i32;
        if(n_idx_a_o_pin > (n_len-1)){
            n_idx_a_o_pin = 0;
        }
        if(n_idx_a_o_pin < 0){
            n_idx_a_o_pin = (n_len-1);
        }
        o_stepper.n_idx_a_o_pin = n_idx_a_o_pin as u8;
        // println!("next substep  n_idx {}", n_idx_a_o_pin);
        for (n_idx, o) in o_stepper.a_o_pin.iter_mut().enumerate(){
            if(n_idx as u8 == o_stepper.n_idx_a_o_pin){
                o.set_high();
            }else{
                o.set_low();
            }
        }
        
    }
    // println!("o_stepper.n_substeps_per_step {}", o_stepper.n_substeps_per_step);                    
    // println!("n_micsec_between_substep {}", n_micsec_between_substep);                    
    // println!("n_rpm {}", n_rpm);                    
    // println!("{:?}", o_stepper);                    
}
fn main() -> Result<(), Box<dyn Error>> {


    // read usb controlle 

    let a_o_input_device = f_a_o_input_device();
    let n_id_vendor = 0x045e;
    let n_id_product = 0x028e; 
    let mut o_input_device = 
        a_o_input_device
            .iter()
            .find(
                |&device|
                device.n_id_vendor == n_id_vendor 
                    && device.n_id_product == n_id_product
            ).expect("could not find device")
            .clone();

    // println!("{:?}", o_input_sensor);
    let devices = rusb::devices()?;

    let mut a_n_u8_read = [0u8; 32];
    let o_duration__timeout = std::time::Duration::from_secs(1);
    // println!("device found {}", o_device);
    let mut o_device_handle = open_device_with_vid_pid(n_id_vendor,n_id_product).unwrap();
    // pub fn open_device_with_vid_pid(
        //     vendor_id: u16,
        //     product_id: u16
        // ) -> Option<DeviceHandle<GlobalContext>>
            // Detach kernel driver if necessary (specific to your device and OS)
    let n_idx_iface = 0;
    let _ = o_device_handle.detach_kernel_driver(n_idx_iface);

    let o = o_device_handle.claim_interface(n_idx_iface);


    // Obtain the GPIO instance
    let o_gpio = Gpio::new()?;

    let o_instant = Instant::now();
    let n_rpm_max = 120.;

    let mut o_stepper_28BYJ_48_x = O_stepper_28BYJ_48{

        a_o_pin : vec![
            o_gpio.get(2).expect("cannot get pin").into_output(),
            o_gpio.get(3).expect("cannot get pin").into_output(),
            o_gpio.get(4).expect("cannot get pin").into_output(),
            o_gpio.get(17).expect("cannot get pin").into_output()
        ],
        n_rpm_nor : 0.01,
        n_rpm_max : n_rpm_max,
        b_direction : true,
        n_substeps: 1,
        n_radians : 0.0,
        n_fullsteps_per_round : 2048,
        n_substeps_per_step: 1, // 2 half stepping
        n_idx_a_o_pin: 0 , 
        n_micsec_sleep_between_fullstep: 0.0, 
        n_micsec_ts_last_step: o_instant.elapsed().as_micros(),
        o_instant: o_instant
    };

    let mut o_stepper_28BYJ_48_y = O_stepper_28BYJ_48{

        a_o_pin : vec![
            o_gpio.get(27).expect("cannot get pin").into_output(),
            o_gpio.get(22).expect("cannot get pin").into_output(),
            o_gpio.get(10).expect("cannot get pin").into_output(),
            o_gpio.get(9).expect("cannot get pin").into_output()
        ],
        n_rpm_nor : 0.01,
        n_rpm_max : n_rpm_max,
        b_direction : true,
        n_substeps: 1,
        n_radians : 0.0,
        n_fullsteps_per_round : 2048,
        n_substeps_per_step: 1, // 2 half stepping
        n_idx_a_o_pin: 0 , 
        n_micsec_sleep_between_fullstep: 0.0, 
        n_micsec_ts_last_step: o_instant.elapsed().as_micros(),
        o_instant: o_instant
    };

    let n_micsec_sleep_probe = 100. as f64;

    let n_idx_o_input_sensor__right_axis_x = o_input_device.a_o_input_sensor.iter()
    .position(|sensor| sensor.s_name == "right_x_axis")
    .expect("Sensor 'right_x_axis' not found");

    let n_idx_o_input_sensor__right_axis_y = o_input_device.a_o_input_sensor.iter()
    .position(|sensor| sensor.s_name == "right_y_axis")
    .expect("Sensor 'right_y_axis' not found");

    let mut n_micsec_last: u128 = o_instant.elapsed().as_micros();
    loop{
        // println!("probe");
        let n_micsec_now = o_instant.elapsed().as_micros();
        let n_micsec_delta = (n_micsec_now - n_micsec_last) as f64;
        println!("micsec delta {}", n_micsec_delta);
        // Perform the interrupt read
        let n_b_read = o_device_handle.read_interrupt(0x81, &mut a_n_u8_read, o_duration__timeout)?;
        f_update_o_input_device(&mut o_input_device, &a_n_u8_read);

        let o_input_sensor__right_axis_x = &o_input_device.a_o_input_sensor[n_idx_o_input_sensor__right_axis_x];
        let o_input_sensor__right_axis_y = &o_input_device.a_o_input_sensor[n_idx_o_input_sensor__right_axis_y];

        o_stepper_28BYJ_48_y.b_direction = o_input_sensor__right_axis_y.n_nor > 0.; 
        o_stepper_28BYJ_48_y.n_rpm_nor = o_input_sensor__right_axis_y.n_nor;
        o_stepper_28BYJ_48_x.b_direction = o_input_sensor__right_axis_x.n_nor > 0.; 
        o_stepper_28BYJ_48_x.n_rpm_nor = o_input_sensor__right_axis_x.n_nor;

        // println!("{:?}", o_input_device);
        // println!("right y axis{}", o_input_sensor__right_axis_y.n_nor);
        f_update_o_stepper(&mut o_stepper_28BYJ_48_y);
        f_update_o_stepper(&mut o_stepper_28BYJ_48_x);

        let n_micsec_probe_diff = n_micsec_sleep_probe - n_micsec_delta;
        println!("probe sleep {}", n_micsec_probe_diff);
        if(n_micsec_probe_diff > 0.){
            thread::sleep(Duration::from_micros(
                (n_micsec_probe_diff as u128).try_into().unwrap()
            ));   
        }
        n_micsec_last = n_micsec_now;

    }




    Ok(())
}
