use rppal::gpio::Gpio;
use std::{error::Error, thread, time::{Duration, SystemTime, UNIX_EPOCH}};
use rusb::{Device, UsbContext, open_device_with_vid_pid};
use core::task::Context;

use crate::classes::O_input_device;

pub mod classes; 


pub mod runtimedata; 
use runtimedata::f_a_o_input_device;

fn f_n_from_string(s: &str) -> usize {
    s.chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<usize>().unwrap()
}

fn f_update_o_input_device(
    o_input_device: &mut O_input_device,
    a_n_u8:  &[u8]//Vec<u8>
) -> bool {

    let mut n_bit = 0;
            for o in &mut o_input_device.a_o_input_sensor {
        let n_idx_byte = n_bit / 8; // e.g., 2
        let n_bits = f_n_from_string(o.s_type); // e.g., 4 for 'u4'
        let mut b_signed = false;
        if o.s_type.contains('i'){
            b_signed = true
        } 

        let n_idx_bit = n_bit % 8; // e.g., 4
        let n_value_max = 2u64.pow(n_bits.try_into().unwrap()) - 1; // e.g., 2^4-1 = 15

        let mut n_value_number = n_value_number;
        if ![8, 16, 32, 64].contains(&n_bits.try_into().unwrap()) {
            n_value_number = n_value_number >> n_idx_bit & n_value_max;
        }
        if o.s_type.contains('i') {
            n_value_max /= 2;
        }

        o.value = Some(n_value_number.try_into().unwrap());
        o.n_nor = n_value_number as f64 / n_value_max as f64;
        if let Some(ref a_o_num_str_value) = o.a_o_num_str_value {
            o.o_num_str_value = a_o_num_str_value.iter().find(|o_enum| o_enum.n == n_value_number);
        }

        n_bit += n_bits;
        let v = if let Some(ref o_num_sta_o_num_str_value) = o.o_num_str_value {
            &o_num_sta_o_num_str_value.s
        } else {
            &o.n_nor
        };
        println!("{:30}: {:?}", o.s_name, v);
    }
    return true
}
fn main() -> Result<(), Box<dyn Error>> {

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



    println!("Global Device {:?}", o_input_device);
    // std::thread::sleep(std::time::Duration::from_millis(10000));

    // println!("{:?}", o_input_sensor);
    let devices = rusb::devices()?;

    let mut a_n_u8_read = [0u8; 64];
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

    let mut n = 0;
    while n < 1000 {
        n+=1;

        // Perform the interrupt read
        let n_b_read = o_device_handle.read_interrupt(0x81, &mut a_n_u8_read, o_duration__timeout)?;
        f_update_o_input_device(&mut o_input_device, &a_n_u8_read);
        println!("{:?}",a_n_u8_read);
    }
    // pub fn claim_interface(&mut self, iface: u8) -> Result<()>

    std::thread::sleep(std::time::Duration::from_millis(20000));

    
    // Obtain the GPIO instance
    let gpio = Gpio::new()?;

    // Select GPIO pin 18
    let mut pin = gpio.get(18)?.into_output();

    let mut n_duty_nor = 0.5;
    let n_micsec_pulse = 5000.;
    let mut n = 0.0;
    loop {
        n+=0.01;
        n_duty_nor = f64::sin(n)*0.5+0.5;
        println!("n_duty_nor {}",   n_duty_nor); // 69420
        // Your code here. For demonstration, we'll just sleep for a short time.
        pin.set_high();
        std::thread::sleep(std::time::Duration::from_micros((n_duty_nor*n_micsec_pulse) as u64));
        // Your code here. For demonstration, we'll just sleep for a short time.
        pin.set_low();
        std::thread::sleep(std::time::Duration::from_micros(((1.-n_duty_nor)*n_micsec_pulse) as u64));
    }


    Ok(())
}
