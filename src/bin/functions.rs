use crate::classes::{
    O_input_device,
    O_input_sensor,
    O_input_sensor_value,
    O_num_str_value
};
use rppal::gpio::Gpio;
use std::{
    self, 
    fs::{
        self,
        create_dir_all,
        remove_file,
        remove_dir_all
    },
    path::Path,
    io,
    sync::{
        Arc, 
        Mutex, 
        mpsc, 
    },
    
    time::{
        Instant,
        Duration
    },
    thread
};
use serde::{
    Deserialize,
    Serialize
};
use serde_json::Value;

use super::classes::A_o_name_synonym;
use crate::classes::O_stepper_28BYJ_48;


pub fn f_create_or_clean_directory(path: &Path) -> io::Result<()> {
    if path.exists() {
        // Iterate over everything in the directory
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                // Recursively remove directories
                remove_dir_all(&path)?;
            } else {
                // Remove files
                remove_file(&path)?;
            }
        }
    } else {
        // Create the directory if it does not exist
        create_dir_all(path)?;
    }
    Ok(())
}

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


fn f_n_normalized(value: i128, bits: u8, signed: bool) -> f64 {
    let min_value: i128;
    let n_range = (1 << bits)-1;

    if signed {
        min_value = -(1 << (bits - 1));
    } else {
        min_value = 0;
    }

    let normalized = (value + min_value.abs()) as f64 / (n_range) as f64;
    // println!("min|range|val|val+abs(min)|nor {}|{}|{}|{}|{}", min_value.abs(),n_range, value, value + min_value.abs(), normalized);
    normalized
}


pub fn f_update_o_input_device(
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

        let mut n_extracted_unsigned = f_n_extracted_unsigned(
            // this simply extracts the bits from start index to startindex+length into an unsigned number
            a_n_u8,
            n_idx_bit_start, 
            n_bits, 
        );

        o.n_nor__last = o.n_nor;

        // now we cast into type according to the number of bits and the sign
        if(n_bits <= 8){
            if(b_signed){
                let n_res_typed = n_extracted_unsigned as i8;
                o.v_o_input_sensor_value = Some(O_input_sensor_value::I8(n_res_typed));
                o.n_nor = f_n_normalized(n_res_typed as i128, n_bits as u8, b_signed);
            }
            if(b_unsigned){
                let n_res_typed = n_extracted_unsigned as u8;
                o.v_o_input_sensor_value = Some(O_input_sensor_value::U8(n_res_typed));
                o.n_nor = f_n_normalized(n_res_typed as i128, n_bits as u8, b_signed);
            }
        }
        if(n_bits > 8 && n_bits <= 16){

            if(b_signed){
                let n_res_typed = n_extracted_unsigned as i16;
                o.v_o_input_sensor_value = Some(O_input_sensor_value::I16(n_res_typed));
                o.n_nor = f_n_normalized(n_res_typed as i128, n_bits as u8, b_signed);
            }
            if(b_unsigned){
                let n_res_typed = n_extracted_unsigned as u16;
                o.v_o_input_sensor_value = Some(O_input_sensor_value::U16(n_res_typed));
                o.n_nor = f_n_normalized(n_res_typed as i128, n_bits as u8, b_signed);
            }
        }
        if(n_bits > 16 && n_bits <= 32){
            if(b_float){
                let n_res_typed = n_extracted_unsigned as f32;
                o.v_o_input_sensor_value = Some(O_input_sensor_value::F32(n_res_typed));
            }
            if(b_signed){
                let n_res_typed = n_extracted_unsigned as i32;
                o.v_o_input_sensor_value = Some(O_input_sensor_value::I32(n_res_typed));
                o.n_nor = f_n_normalized(n_res_typed as i128, n_bits as u8, b_signed);
            }
            if(b_unsigned){
                let n_res_typed = n_extracted_unsigned as u32;
                o.v_o_input_sensor_value = Some(O_input_sensor_value::U32(n_res_typed));
                o.n_nor = f_n_normalized(n_res_typed as i128, n_bits as u8, b_signed);
            }
        }
        if(n_bits > 32 && n_bits <= 64){
            if(b_float){
                let n_res_typed = n_extracted_unsigned as f64;
                o.v_o_input_sensor_value = Some(O_input_sensor_value::F64(n_res_typed));
            }
            if(b_signed){
                let n_res_typed = n_extracted_unsigned as i64;
                o.v_o_input_sensor_value = Some(O_input_sensor_value::I64(n_res_typed));
                o.n_nor = f_n_normalized(n_res_typed as i128, n_bits as u8, b_signed);
            }
            if(b_unsigned){
                let n_res_typed = n_extracted_unsigned as u64;
                o.v_o_input_sensor_value = Some(O_input_sensor_value::U64(n_res_typed));
                o.n_nor = f_n_normalized(n_res_typed as i128, n_bits as u8, b_signed);
            }
        }
        
        
        // we treat nor as normalized and thus always from 0.0 to 1.0, 
        // if the number is signed it would be 'snor'
        // this is because in gamepad usb controllers for example 
        // not all controllers use a signed number for stick axis, then the value is 0.5 in the middle 
        // to make it easier we only provide a normalized value, the programmer then can get the information
        // if a value is signed or not by looking at the 's_type' which can be 'u8' or 'i8' etc.
        // if(b_signed){
        //     o.n_nor +=0.5;
        // }

        o.v_o_num_str_value = o.a_o_num_str_value.iter().find(|&e| e.n == n_extracted_unsigned).cloned();


        n_idx_bit_start += n_bits as usize;
    }
}
pub fn f_o_input_sensor_from_s_name<'a>(o_input_device: &'a O_input_device, s_name: &str) -> Option<&'a O_input_sensor> {
    match o_input_device.a_o_input_sensor.iter().find(|o_input_sensor| o_input_sensor.s_name == s_name) {
        Some(o_input_sensor) => Some(o_input_sensor),
        None => {
            println!("Error: o_input_sensor '{}' not found", s_name); // Print error message here
            None
        }
    }
}

pub fn f_b_bool_button_down(
    o_input_device: &mut O_input_device,
    a_o_name_synonym: &A_o_name_synonym,
    s_name: &str, 
)-> bool{
    // todo...
    let mut s_name_to_find = s_name.to_string();
    println!("s_name_to_find {}", s_name_to_find);
    
    for o_name_synonym in a_o_name_synonym {
        if o_name_synonym.a_s_synonym.contains(&s_name_to_find) {
            s_name_to_find =  o_name_synonym.s_name.clone();
            // println!("found {}", s_name_to_find);
            // std::process::exit(0);
            break;
        }
    }


    for o_input_sensor in &o_input_device.a_o_input_sensor{
        if(o_input_sensor.v_o_num_str_value.is_some()){
            let o_num_str_value = o_input_sensor.v_o_num_str_value.clone().unwrap();

            if(o_num_str_value.a_s_name.contains(&s_name_to_find)){
                return true;
                // v = Some(1.0);
                // println!("found {:?}", v);
                // std::process::exit(0);
            }
        }

    }
    return false
}



pub fn f_n_extracted_unsigned(
    a_n_u8: &[u8],
    n_idx_bit_start: usize,
    n_bits: usize
) -> u64 {
    // this simply extracts the bits at start index into a resulting unsigned number they fit in, 
    // for exampel 
    //  2 bits -> u8
    //  8 bits -> u8
    // 12 bits -> u16
    // 15 bits -> u16
    // 64 bits -> u64 
    // etc...

    // assert!(!b_float, "Floating point not supported directly."); // Simplification for this example

    let mut n_res: u64 = 0;
    let mut n_idx_bit_in_extracted = 0;

    while n_idx_bit_in_extracted < n_bits {
        let n_idx_byte = (n_idx_bit_start + n_idx_bit_in_extracted) / 8;
        let n_idx_bit_modulo = (n_idx_bit_start + n_idx_bit_in_extracted) % 8;
        let n_value_bit = (a_n_u8[n_idx_byte] >> n_idx_bit_modulo) & 1;

        n_res = n_res | ((n_value_bit as u64) << n_idx_bit_in_extracted);
        n_idx_bit_in_extracted += 1;
    }

    n_res
}

pub fn f_substep_o_stepper(
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
    // // debug stepping
    // println!("a_o_pin");
    // for o_pin in  o_stepper.a_o_pin.iter(){
    //     print!("{:?}", if(o_pin.is_set_low()){0}else{1})
    // }
    
    

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

pub fn f_b_gpio_available(

)->bool{
    return Gpio::new().is_ok();
}
pub fn f_o_stepper_28BYJ_48(
    a_n_pin: [u8;4],
)->O_stepper_28BYJ_48{

    let o_gpio = Gpio::new().expect("cannot instaniate GPIO is this programm running on a raspberry pi ?");
    let o_instant = Instant::now();

    let mut o_stepper = O_stepper_28BYJ_48{

        a_o_pin : vec![
            o_gpio.get(a_n_pin[0]).expect("cannot get pin").into_output(),
            o_gpio.get(a_n_pin[1]).expect("cannot get pin").into_output(),
            o_gpio.get(a_n_pin[2]).expect("cannot get pin").into_output(),
            o_gpio.get(a_n_pin[3]).expect("cannot get pin").into_output()
        ],
        b_depower_if_rpm_zero: true,
        n_rpm_nor : 0.5,
        n_rpm_nor_last : 0.5,
        n_rpm_max : 15.,
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

    return o_stepper
}

pub fn f_o_mutex_arc_o_stepper_28BYJ_48(
    a_n_pin: [u8;4]
)-> (Arc<Mutex<O_stepper_28BYJ_48>>, thread::JoinHandle<()>){

    // Obtain the GPIO instance
    let o_gpio = Gpio::new().expect("cannot instaniate GPIO is this programm running on a raspberry pi ?");
    let o_instant = Instant::now();

    let mut o_stepper = O_stepper_28BYJ_48{

        a_o_pin : vec![
            o_gpio.get(a_n_pin[0]).expect("cannot get pin").into_output(),
            o_gpio.get(a_n_pin[1]).expect("cannot get pin").into_output(),
            o_gpio.get(a_n_pin[2]).expect("cannot get pin").into_output(),
            o_gpio.get(a_n_pin[3]).expect("cannot get pin").into_output()
        ],
        b_depower_if_rpm_zero: true,
        n_rpm_nor : 0.5,
        n_rpm_nor_last : 0.5,
        n_rpm_max : 15.,
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
    // the stepper could be driven even faster
    let n_rpm_absolute_max = 22.;
    let mut n_micsec_sleep_probe = (1000.0*1000.0*60.0)/ (n_rpm_absolute_max * o_stepper.n_substeps_per_step as f64 * o_stepper.n_fullsteps_per_round as f64);
    // there is a law that we need 2 samples per sample or so 
    n_micsec_sleep_probe = n_micsec_sleep_probe/2.;
    
    // println!("n_micsec_sleep_probe {}", n_micsec_sleep_probe);
    // std::process::exit(0);

    // Wrap your struct in a Mutex and then an Arc
    let o_mutex_arc = Arc::new(Mutex::new(o_stepper));

    // Clone the Arc to move it into the thread
    let o_mutex_arc_clone = o_mutex_arc.clone();

    let mut n_micsec_last: u128 = o_instant.elapsed().as_micros();
    

    // Spawn a new thread
    let o_thread_handle = thread::spawn(move || {
        
        loop{
            
            let mut o_stepper = o_mutex_arc_clone.lock().unwrap();
            let n_micsec_now = o_instant.elapsed().as_micros();
            let n_micsec_delta = (n_micsec_now - n_micsec_last) as f64;
            // println!("micsec delta {}", n_micsec_delta);
            
            f_check_mic_sec_delta_and_potentially_step(&mut o_stepper);
    
            let n_micsec_probe_diff = n_micsec_sleep_probe - n_micsec_delta;
            // println!("probe sleep {}", n_micsec_probe_diff);
            if(n_micsec_probe_diff > 0.){
                thread::sleep(Duration::from_micros(
                    (n_micsec_probe_diff as u128).try_into().unwrap()
                ));   
            }
            n_micsec_last = n_micsec_now;
    
        }

    });

    return (o_mutex_arc, o_thread_handle);

}

pub fn f_o_sender_tx_spawn_thread_with_event_listener_for_stepper(
    a_n_pin: [u8;4]
)->mpsc::Sender::<String>{

    let (o_sender_tx, o_receiver_rx) = mpsc::channel::<String>();

    // Obtain the GPIO instance
    let o_gpio = Gpio::new().expect("cannot instaniate GPIO is this programm running on a raspberry pi ?");
    let o_instant = Instant::now();

    let mut o_stepper = O_stepper_28BYJ_48{

        a_o_pin : vec![
            o_gpio.get(a_n_pin[0]).expect("cannot get pin").into_output(),
            o_gpio.get(a_n_pin[1]).expect("cannot get pin").into_output(),
            o_gpio.get(a_n_pin[2]).expect("cannot get pin").into_output(),
            o_gpio.get(a_n_pin[3]).expect("cannot get pin").into_output()
        ],
        b_depower_if_rpm_zero: true,
        n_rpm_nor : 0.0,
        n_rpm_nor_last : 0.5,
        n_rpm_max : 15.,
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
    // the stepper could be driven even faster
    let n_rpm_absolute_max = 22.;
    let mut n_micsec_sleep_probe = (1000.0*1000.0*60.0)/ (n_rpm_absolute_max * o_stepper.n_substeps_per_step as f64 * o_stepper.n_fullsteps_per_round as f64);
    // there is a law that we need 2 samples per sample or so 
    n_micsec_sleep_probe = n_micsec_sleep_probe/2.;
    
    // println!("n_micsec_sleep_probe {}", n_micsec_sleep_probe);
    // std::process::exit(0);

    let mut n_micsec_last: u128 = o_instant.elapsed().as_micros();
    

    // Spawn a new thread
    let o_thread_handle = thread::spawn(move || {
        loop {
            match o_receiver_rx.try_recv() {
                Ok(s_msg) => {
                    let value: Value = serde_json::from_str(&s_msg).unwrap();
                    if let Some(n_rpm) = value.get("n_rpm_nor") {
                        let v = n_rpm.as_f64().unwrap();
                        // println!("property found: {:?}", v);
                        o_stepper.n_rpm_nor = v;
                    }
                    if let Some(n_rpm) = value.get("b_direction") {
                        let v = n_rpm.as_bool().unwrap();
                        o_stepper.b_direction = v;
                    }
                    if let Some(n_rpm) = value.get("n_rpm_max") {
                        let v = n_rpm.as_f64().unwrap();
                        o_stepper.n_rpm_max = v;
                    }
                    if let Some(n_rpm) = value.get("n_substeps_per_step") {
                        let v = n_rpm.as_u64().unwrap();
                        o_stepper.n_substeps_per_step = v as u32;
                    }
                    if let Some(n_rpm) = value.get("b_depower_if_rpm_zero") {
                        let v = n_rpm.as_bool().unwrap();
                        o_stepper.b_depower_if_rpm_zero = v;
                    }
                }
                Err(e) => {
                    // println!("Error or empty message,  receiving message: {:?}", e);
                    // Handle the error or continue the loop
                }
            }
        
            // Rest of the loop

            let n_micsec_now = o_instant.elapsed().as_micros();
            let n_micsec_delta = (n_micsec_now - n_micsec_last) as f64;
            // println!("micsec delta {}", n_micsec_delta);
            
            f_check_mic_sec_delta_and_potentially_step(&mut o_stepper);
    
            let n_micsec_probe_diff = n_micsec_sleep_probe - n_micsec_delta;
            // println!("probe sleep {}", n_micsec_probe_diff);
            if(n_micsec_probe_diff > 0.){
                thread::sleep(Duration::from_micros(
                    (n_micsec_probe_diff as u128).try_into().unwrap()
                ));   
            }
            n_micsec_last = n_micsec_now;
    
        }


    });

    return o_sender_tx;

}

pub fn f_update_o_stepper_recalculate_micsecs(
    o_stepper: &mut O_stepper_28BYJ_48, 
){

    let n_rpm = o_stepper.n_rpm_nor.abs() * o_stepper.n_rpm_max;
    let n_fullsteps_per_minute = o_stepper.n_fullsteps_per_round as f64 * n_rpm; 
    o_stepper.n_micsec_sleep_between_fullstep = (60*1000*1000) as f64 / n_fullsteps_per_minute;


}

pub fn f_check_mic_sec_delta_and_potentially_step(
    o_stepper: &mut O_stepper_28BYJ_48
){

    let n_micsec_between_substep = (o_stepper.n_micsec_sleep_between_fullstep as f64) / o_stepper.n_substeps_per_step as f64;
    let n_micsec_ts_now = o_stepper.o_instant.elapsed().as_micros();
    f_update_o_stepper_recalculate_micsecs(o_stepper);
    
    if(o_stepper.b_depower_if_rpm_zero){

        if(o_stepper.n_rpm_nor == 0.0 && o_stepper.n_rpm_nor_last != 0.0){
            for o_pin in  o_stepper.a_o_pin.iter_mut(){
                o_pin.set_low()
            }
        }
    }

    if(o_stepper.n_rpm_nor == 0.0){
        return;
    }
        // println!("micsec elapsed {}", n_micsec_ts_now);
    // println!("micsec delta {}", n_micsec_ts_now - o_stepper.n_micsec_ts_last_step);
    if(
        (n_micsec_ts_now - o_stepper.n_micsec_ts_last_step) > n_micsec_between_substep as u128
    ){
        f_substep_o_stepper(o_stepper);
        
    }
    // println!("o_stepper.n_substeps_per_step {}", o_stepper.n_substeps_per_step);                    
    // println!("n_micsec_between_substep {}", n_micsec_between_substep);                    
    // println!("n_rpm {}", n_rpm);                    
    // println!("{:?}", o_stepper);                    
}

pub fn f_step_degrees_nor(
    o_stepper: &mut O_stepper_28BYJ_48, 
    n_degrees_nor: f32
){
    let n_substeps = o_stepper.n_fullsteps_per_round as f32 * (o_stepper.n_substeps_per_step as f32) * n_degrees_nor;
     
    let o_instant = Instant::now();
    let n_rpm_max = 10.;
    
    f_update_o_stepper_recalculate_micsecs(o_stepper);

    let n_micsec_between_substep = (o_stepper.n_micsec_sleep_between_fullstep as f64) / o_stepper.n_substeps_per_step as f64;

    for n in 0..(n_substeps as i32){

        f_substep_o_stepper(o_stepper);

        // we hope that the thread sleep function is accurate, for a 
        // byj28 stepper 
        thread::sleep(Duration::from_micros(
            (n_micsec_between_substep as u64).try_into().unwrap()
        ));

    }

}

// println!("{:?}", o_input_sensor);