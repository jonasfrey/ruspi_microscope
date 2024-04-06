use rppal::gpio::Gpio;
use std::{error::Error, thread, time::{Duration, SystemTime, UNIX_EPOCH}};
use rusb::{Device, UsbContext, open_device_with_vid_pid};
use core::task::Context;


fn main() -> Result<(), Box<dyn Error>> {

    let devices = rusb::devices()?;

    // for device in devices.iter() {
    //     let device_desc = device.device_descriptor()?;

    //     let handle = match device.open() {
    //         Ok(handle) => handle,
    //         Err(_) => continue, // Skip devices that cannot be opened.
    //     };

    //     let manufacturer = handle.read_manufacturer_string_ascii(&device_desc)?;
    //     let product = handle.read_product_string_ascii(&device_desc)?;
    //     if(device_desc.vendor_id() == 0x045e
    //         && device_desc.product_id() == 0x028e
    //     ){
    //         o_device = device
    //     }
    //     // println!("Manufacturer: {}, Product: {}", manufacturer, product);

    //     println!("Bus {:03} Device {:03} ID {:04x}:{:04x} {} {}",
    //              device.bus_number(),
    //              device.address(),
    //              device_desc.vendor_id(),
    //              device_desc.product_id(),
    //              manufacturer,
    //                 product
    //         );

    // }
    let mut a_n_u8_read = [0u8; 64];
    let o_duration__timeout = std::time::Duration::from_secs(1);
    // println!("device found {}", o_device);
    let mut o_device_handle = open_device_with_vid_pid(0x045e,0x028e).unwrap();
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
