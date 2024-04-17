use classes::O_input_sensor_value;
use rppal::gpio::Gpio;
use data_url::{DataUrl, mime};
use serde_json::{Value, json};
use std::{error::Error, os::unix::process, process::exit, sync::{Arc}, thread::{self, JoinHandle}, time::{Duration, Instant, SystemTime, UNIX_EPOCH}};
use rusb::{Device, UsbContext, DeviceHandle, open_device_with_vid_pid};
use core::task::Context;
use std::process::Command;
use image::ImageFormat;
use base64::decode;
use image::save_buffer;
use crate::classes::O_input_sensor;
use tokio::sync::{Mutex, mpsc};
use tokio::sync::mpsc::error::TryRecvError;
// use tokio::sync::mpsc;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use futures::SinkExt;
use tokio_tungstenite::accept_async;
// use tokio::stream::StreamExt;
use tungstenite::protocol::Message;
// use futures_util::stream::stream::StreamExt;
use futures::stream::StreamExt;
use crate::classes::{O_input_device, O_stepper_28BYJ_48};
use tokio::runtime::Runtime;
use warp::Filter;
use std::fs::File;
pub mod classes; 
use image::load_from_memory;


pub mod runtimedata; 
use runtimedata::f_a_o_input_device;
fn f_n_u8_sum_wrap(
    n_u8: u8, 
    n_u8_idx_max: u8, 
    n_i8_summand : i8,  
    // max = 4, summand = +1 -> 0, 1, 2, 3, 0, 1, 2, 3, 0...
    // max = 4, summand = -1 -> 0, 3, 2, 1, 0, 3, 2, 1, 0...
) -> u8 {
    let n_res = n_u8 as i16 + n_i8_summand as i16;
    if(n_res < 0){
        return n_u8_idx_max-1;
    } 
    return (n_res % n_u8_idx_max as i16).try_into().unwrap()
}
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
        o.n_nor__last = o.n_nor;
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



async fn f_start_usb_read_thread<C: UsbContext + 'static>(
    device_handle: Arc<Mutex<DeviceHandle<C>>>,
    timeout: Duration,
) -> mpsc::Receiver<Vec<u8>> {
    // Create a channel for communicating USB read results back to the main thread
    let (tx, mut rx) = tokio::sync::mpsc::channel(32); // Adjust channel size as needed

    tokio::spawn(async move {
        loop {
            let mut buffer = vec![0u8; 32]; // Adjust buffer size as needed
            {
                let mut o_device_handle = device_handle.lock().await;
                let n_idx_iface = 0;
                let _ = o_device_handle.detach_kernel_driver(n_idx_iface).ok();
                let _ = o_device_handle.claim_interface(n_idx_iface).ok();
            }
            {
                let handle = device_handle.lock().await;
                match handle.read_interrupt(0x81, &mut buffer, timeout) {
                    Ok(bytes_read) => {
                        println!("Read from USB device success");
                        buffer.truncate(bytes_read); // Adjust buffer size to actual bytes read
                        if tx.send(buffer.clone()).await.is_err() {
                            eprintln!("Failed to send data through channel");
                            break;
                        }
                    },
                    Err(e) => {
                        eprintln!("USB read error: {:?}", e);
                        break;
                    }
                }
            }
            tokio::time::sleep(Duration::from_millis(1)).await; // Adjust based on your requirements
        }
    });

    rx
}


fn f_substep_o_stepper(
    o_stepper: &mut O_stepper_28BYJ_48
){
    let n_dir = if(o_stepper.b_direction){ 1 }else{ -1};

    let n_len_a_o_pin = o_stepper.a_o_pin.len(); 
    o_stepper.n_idx_substep = f_n_u8_sum_wrap(
        o_stepper.n_idx_substep,
        ((n_len_a_o_pin as u32 * o_stepper.n_substeps_per_step)).try_into().unwrap(),
        n_dir as i8);   
    // println!("n_idx_substep {}", o_stepper.n_idx_substep);                    
    // next sub step

    o_stepper.n_micsec_ts_last_step = o_stepper.o_instant.elapsed().as_micros();;
    o_stepper.n_substeps+=1;

    let mut n_idx_a_o_pin = (o_stepper.n_idx_substep as f32 / o_stepper.n_substeps_per_step as f32) as usize; 
    {
        let mut o_pin = &mut o_stepper.a_o_pin[n_idx_a_o_pin];
        o_pin.set_high();
    }
    let n_mod = if(n_dir == 1) { 1} else {0};
    if(o_stepper.n_idx_substep % o_stepper.n_substeps_per_step as u8 == n_mod){
        let mut o_pin_last = &mut o_stepper.a_o_pin[
            f_n_u8_sum_wrap(
                n_idx_a_o_pin.try_into().unwrap(),
                n_len_a_o_pin.try_into().unwrap(),
                n_dir*-1
            ) as usize
        ];
        o_pin_last.set_low();
    }
    // println!("n_idx_a_o_pin {}", n_idx_a_o_pin);                    
    

    // 2 substeps
    // 1 0 0 0 
    // 1 1 0 0
    // 0 1 0 0
    // 0 1 1 0
    // 0 0 1 0 
    // 0 0 1 1
    // 0 0 0 1
    // 1 0 0 1 
    // 1 0 0 0
}
fn f_check_mic_sec_delta_and_potentially_step(
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
        f_substep_o_stepper(o_stepper);
        
    }
    // println!("o_stepper.n_substeps_per_step {}", o_stepper.n_substeps_per_step);                    
    println!("n_micsec_between_substep {}", n_micsec_between_substep);                    
    // println!("n_rpm {}", n_rpm);                    
    // println!("{:?}", o_stepper);                    
}
fn f_o_input_sensor_from_s_name<'a>(a_o_input_sensor: &'a [O_input_sensor], s_name: &str) -> Option<&'a O_input_sensor> {
    match a_o_input_sensor.iter().find(|o_input_sensor| o_input_sensor.s_name == s_name) {
        Some(o_input_sensor) => Some(o_input_sensor),
        None => {
            println!("Error: o_input_sensor '{}' not found", s_name); // Print error message here
            None
        }
    }
}
/// Parses a data URL and extracts the MIME type and encoded data.
fn parse_data_url(data_url: &str) -> Option<(&str, &str)> {
    let parts: Vec<&str> = data_url.splitn(2, ',').collect();
    if parts.len() == 2 {
        let metadata = parts[0];
        let encoded_data = parts[1];
        let metadata_parts: Vec<&str> = metadata.split(';').collect();
        if metadata_parts.len() == 2 && metadata_parts[1] == "base64" {
            return Some((metadata_parts[0].strip_prefix("data:").unwrap_or(""), encoded_data));
        }
    }
    None
}

async fn save_image(data: &[u8]) {
    match image::load_from_memory(data) {
        Ok(img) => {
            let mut output = File::create("output.jpg").expect("Failed to create file");
            img.write_to(&mut output, ImageFormat::Jpeg).expect("Failed to write image");
            println!("Image saved as output.jpg");
        },
        Err(e) => {
            eprintln!("Error processing image: {:?}", e);
        }
    }
}

struct O_test{
    n: u8
}
async fn handle_connection(raw_stream: TcpStream, state: Arc<Mutex<O_test>>) {
    let ws_stream = accept_async(raw_stream).await.expect("Failed to accept");
    let (mut write, mut read) = ws_stream.split();

    // Spawn a task to handle incoming messages
    let read_task = tokio::spawn(async move {
        while let Some(message) = read.next().await {
            match message {
                Ok(msg) => {
                    if let Message::Text(text) = msg {
                        let mut stepper = state.lock().await;
                        // Modify stepper based on text
                        // e.g., parse command and apply to stepper
                        println!("Received via WebSocket: {}", text);
                        let s_b64_image = text;
                        let url = DataUrl::process(&s_b64_image).unwrap();
                        let (body, fragment) = url.decode_to_vec().unwrap();
                        println!("body {:?}", body);
                        let img = load_from_memory(&body).expect("Failed to load image from memory");
                        let output_path = "./test.png";
                        img.save(output_path).expect("Failed to save image");


                        //     // Parse the JSON string
                        // // let v =  serde_json::from_str::<Value>(&text);
                        // let v: Value = serde_json::from_str(&text).expect("cannot parse json");

                        // if v.get("s_b64_image").is_some() {
                        //     let s_b64_image = v["s_b64_image"].to_string();
   
                        //     // Strip the prefix off
                        //     let prefix = "data:image/jpeg;base64,";
                        //     let output_path = "./tmp.jpeg";
                        //     let base64_data = &s_b64_image[prefix.len()..];

                        //     // Decode the base64 data to bytes
                        //     let image_data = decode(base64_data).expect("failed to decode");
                        
                        //     // Save the decoded bytes as an image
                        //     save_buffer(output_path, &image_data, 100, 100, image::ColorType::Rgb8);

                        //     // println!("s_b64_image {}", s_b64_image);
                        //     // let url = DataUrl::process(&s_b64_image).unwrap();
                        //     // let (body, fragment) = url.decode_to_vec().unwrap();
                        //     // println!("body {:?}", body);
                        //     // let image_data = decode(encoded_data).expect("Failed to decode base64");
                        //     // let img = load_from_memory(&image_data).expect("Failed to load image from memory");
                        //     // let output_path = "./test.png";
                        //     // img.save(output_path).expect("Failed to save image");

                        // }

                        
                        // if v.get("o_usb_device").is_some() {

                        //     let n_id_vendor = v["o_usb_device"]["n_id_vendor"].as_u64().unwrap() as u16;
                        //     let n_id_product = v["o_usb_device"]["n_id_vendor"].as_u64().unwrap() as u16;
    
                        //     let mut o_device_handle = open_device_with_vid_pid(
                        //         1133,// n_id_vendor, 
                        //         49948// n_id_product
                        //     ).unwrap();
                            
                        //     // Start the USB read thread
                        //     let o_arc_mutex_o_device_handle = Arc::new(Mutex::new(o_device_handle));
                        //     let timeout = Duration::from_millis(100);
                            
                        //     let usb_read_receiver = f_start_usb_read_thread(o_arc_mutex_o_device_handle, timeout);
                        // }

                    }
                }
                Err(e) => {
                    eprintln!("WebSocket error: {:?}", e);
                    break;
                }
            }
        }
    });

    // Continuously send messages at a fixed interval
    let mut interval = tokio::time::interval(Duration::from_millis(1000));
    loop {
        interval.tick().await;  // Wait for the next interval tick
        // Prepare your message
        let message = Message::Text("Periodic message from server".to_string());
        let output = Command::new("lsusb")
            .output().expect("cannot run lsusb")
            .stdout;
        let sdev = (String::from_utf8_lossy(&output).to_string());

        let json_output = json!({ "s_stdout__lsusb": sdev });
        let json_string = json_output.to_string();

        // println!("message {:?}", sdev);
        if write.send(Message::Text(json_string)).await.is_err() {
            eprintln!("Failed to send message");
            break;
        }
    }

    // Await the reader task to finish (if it finishes)
    let _ = read_task.await;
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {



    thread::spawn(|| {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let route = warp::fs::dir("public");
            warp::serve(route)
                .run(([127, 0, 0, 1], 3030))
                .await;
        });
    });



    let o_test_arc_mutex = Arc::new(Mutex::new(O_test {
        // Initialize your stepper struct
        n: 2
    }));

    let listener = TcpListener::bind("127.0.0.1:9000").await?;
    while let Ok((stream, _)) = listener.accept().await {
        let state = o_test_arc_mutex.clone();
        tokio::spawn(handle_connection(stream, state));
    }


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

    
    // Start the USB read thread
    let o_arc_mutex_o_device_handle = Arc::new(Mutex::new(o_device_handle));
    let timeout = Duration::from_millis(100);
    
    let usb_read_receiver = f_start_usb_read_thread(o_arc_mutex_o_device_handle, timeout);
    
    
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

    // Obtain the GPIO instance
    let o_gpio = Gpio::new()?;

    let o_instant = Instant::now();
    let n_rpm_max = 10.;

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
        n_idx_substep: 0,
        n_radians : 0.0,
        n_fullsteps_per_round : 2048,
        n_substeps_per_step: 2,//1,//2, // 2 half stepping
        n_micsec_sleep_between_fullstep: 0.0, 
        n_micsec_ts_last_step: o_instant.elapsed().as_micros(),
        o_instant: o_instant
    };

    let mut o_stepper_28BYJ_48_y = O_stepper_28BYJ_48{

        a_o_pin : vec![
            o_gpio.get(6).expect("cannot get pin").into_output(),
            o_gpio.get(13).expect("cannot get pin").into_output(),
            o_gpio.get(19).expect("cannot get pin").into_output(),
            o_gpio.get(26).expect("cannot get pin").into_output()
        ],
        n_rpm_nor : 0.01,
        n_rpm_max : n_rpm_max,
        b_direction : true,
        n_substeps: 1,
        n_idx_substep: 0,
        n_radians : 0.0,
        n_fullsteps_per_round : 2048,
        n_substeps_per_step: 2,//1,//2, // 2 half stepping
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
    let n_idx_o_input_sensor__r1 = o_input_device.a_o_input_sensor.iter()
    .position(|sensor| sensor.s_name == "r1")
    .expect("Sensor 'r1' not found");

    // cross_button
    // circle_button


    let mut n_micsec_last: u128 = o_instant.elapsed().as_micros();
    loop{
        // println!("probe");
        let n_micsec_now = o_instant.elapsed().as_micros();
        let n_micsec_delta = (n_micsec_now - n_micsec_last) as f64;
        // println!("micsec delta {}", n_micsec_delta);

        // Perform the interrupt read, which would take around 8000 microsecs so we run a thread for it 
        // let n_b_read = o_device_handle.read_interrupt(0x81, &mut a_n_u8_read, o_duration__timeout)?;
        // f_update_o_input_device(&mut o_input_device, &a_n_u8_read);
        let o_input_sensor__right_x_axis =  f_o_input_sensor_from_s_name(&o_input_device.a_o_input_sensor, "right_x_axis" ).unwrap();
        let o_input_sensor__right_y_axis =  f_o_input_sensor_from_s_name(&o_input_device.a_o_input_sensor, "right_y_axis" ).unwrap();
        let o_input_sensor__r1 = f_o_input_sensor_from_s_name(&o_input_device.a_o_input_sensor, "r1" ).unwrap();
        let n_factor = if(o_input_sensor__r1.n_nor == 1.){ 0.1}else{1.};
        
        o_stepper_28BYJ_48_y.b_direction = o_input_sensor__right_y_axis.n_nor > 0.; 
        o_stepper_28BYJ_48_y.n_rpm_nor = o_input_sensor__right_y_axis.n_nor*n_factor;
        o_stepper_28BYJ_48_x.b_direction = o_input_sensor__right_x_axis.n_nor > 0.; 
        o_stepper_28BYJ_48_x.n_rpm_nor = o_input_sensor__right_x_axis.n_nor*n_factor;

        // println!("{:?}", o_input_device);
        // println!("right y axis{}", o_input_sensor__right_y_axis.n_nor);
        f_check_mic_sec_delta_and_potentially_step(&mut o_stepper_28BYJ_48_y);
        f_check_mic_sec_delta_and_potentially_step(&mut o_stepper_28BYJ_48_x);

        // while let Some(a_n_u8_read) = usb_read_receiver.await.recv().await {
        //     // Process the data received from the USB device
        //     println!("Received USB data: {:?}", a_n_u8_read);
        //     // Further processing...
        // }

        // match usb_read_receiver.await.try_recv() {
        //     Ok(a_n_u8_read) => {
        //         // Process the data received from the USB device
        //         // println!("Received USB data: {:?}", a_n_u8_read);
            
                
        //         f_update_o_input_device(&mut o_input_device, &a_n_u8_read);
        //         let o_input_sensor__d_pad_left = f_o_input_sensor_from_s_name(&o_input_device.a_o_input_sensor, "d_pad_left").unwrap();
        //         let o_input_sensor__d_pad_right = f_o_input_sensor_from_s_name(&o_input_device.a_o_input_sensor, "d_pad_right").unwrap();

        //         if(o_input_sensor__d_pad_right.n_nor == 1. && o_input_sensor__d_pad_right.n_nor__last != 1.){
        //             o_stepper_28BYJ_48_x.b_direction = true; 
        //             f_substep_o_stepper(&mut o_stepper_28BYJ_48_x)
        //         }
        //         if(o_input_sensor__d_pad_left.n_nor == 1. && o_input_sensor__d_pad_left.n_nor__last != 1.){
        //             o_stepper_28BYJ_48_x.b_direction = false; 
        //             f_substep_o_stepper(&mut o_stepper_28BYJ_48_x)
        //         }
        //     }
        //     Err(tokio::sync::mpsc::error::TryRecvError::Empty) => {
        //         // No data available yet; the main thread can perform other work
        //     }
        //     Err(e) => {
        //         // Handle other kinds of errors (e.g., the sender was disconnected)
        //         // eprintln!("Channel receive error: {:?}", e);
        //         break;
        //     }
        // }





        let n_micsec_probe_diff = n_micsec_sleep_probe - n_micsec_delta;
        // println!("probe sleep {}", n_micsec_probe_diff);
        if(n_micsec_probe_diff > 0.){
            thread::sleep(Duration::from_micros(
                (n_micsec_probe_diff as u128).try_into().unwrap()
            ));   
        }
        n_micsec_last = n_micsec_now;

    }




    Ok(())
}
