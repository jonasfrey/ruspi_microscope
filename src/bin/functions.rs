use crate::classes::{
    O_input_device,
    O_input_sensor,
    O_input_sensor_value,
    O_num_str_value
};

use crate::classes::A_o_name_synonym;

fn f_n_from_string(s: &str) -> u32 {
    s.replace(|c: char| !c.is_digit(10), "").parse::<u32>().unwrap_or(0)
}

fn f_convert_endianess(n_res: u64, bit_length: usize) -> u64 {
    match bit_length {
        16 => u16::from_be_bytes(n_res.to_be_bytes()[6..8].try_into().unwrap()) as u64,
        32 => u32::from_be_bytes(n_res.to_be_bytes()[4..8].try_into().unwrap()) as u64,
        64 => u64::from_be_bytes(n_res.to_be_bytes()),
        _ => n_res, // No conversion needed for bit lengths that don't match multi-byte data types
    }
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
        let mut n_value_max = (1u64 << n_bits) - 1;
        o.n_nor__last = o.n_nor;
        if o.s_type.contains('i') {
            n_value_max = n_value_max / 2;
            // Handle signed integers if needed
        }
        if(n_bits <= 8){
            // skipping because f8 not existing in this case
            // if(b_float){
            //     o.v_o_input_sensor_value = Some(O_input_sensor_value::F8(n_res as f8))
            // }
            if(b_signed){
                o.v_o_input_sensor_value = Some(O_input_sensor_value::I8(n_res as i8));
                o.n_nor = (n_res as i8) as f64 / n_value_max as f64;
            }
            if(b_unsigned){
                o.v_o_input_sensor_value = Some(O_input_sensor_value::U8(n_res as u8));
                o.n_nor = (n_res as u8) as f64 / n_value_max as f64;

            }
        }
        if(n_bits > 8 && n_bits <= 16){
            // skipping because f16 not existing in this case
            // if(b_float){
            //     o.v_o_input_sensor_value = Some(O_input_sensor_value::f16(n_res as f16))
            // }
            if(b_signed){
                o.v_o_input_sensor_value = Some(O_input_sensor_value::I16(n_res as i16));
                o.n_nor = (n_res as i16) as f64 / n_value_max as f64;

            }
            if(b_unsigned){
                o.v_o_input_sensor_value = Some(O_input_sensor_value::U16(n_res as u16));
                o.n_nor = (n_res as u16) as f64 / n_value_max as f64;

            }
        }
        if(n_bits > 16 && n_bits <= 32){
            if(b_float){
                o.v_o_input_sensor_value = Some(O_input_sensor_value::F32(n_res as f32));
                o.n_nor = (n_res as f32) as f64 / n_value_max as f64;
            }
            if(b_signed){
                o.v_o_input_sensor_value = Some(O_input_sensor_value::I32(n_res as i32));
                o.n_nor = (n_res as i32) as f64 / n_value_max as f64;
            }
            if(b_unsigned){
                o.v_o_input_sensor_value = Some(O_input_sensor_value::U32(n_res as u32));
                o.n_nor = (n_res as u32) as f64 / n_value_max as f64;
            }
        }
        if(n_bits > 32 && n_bits <= 64){
            if(b_float){
                o.v_o_input_sensor_value = Some(O_input_sensor_value::F64(n_res as f64));
                o.n_nor = (n_res as f64) as f64 / n_value_max as f64;
            }
            if(b_signed){
                o.v_o_input_sensor_value = Some(O_input_sensor_value::I64(n_res as i64));
                o.n_nor = (n_res as i64) as f64 / n_value_max as f64;
            }
            if(b_unsigned){
                o.v_o_input_sensor_value = Some(O_input_sensor_value::U64(n_res as u64));
                o.n_nor = (n_res as u64) as f64 / n_value_max as f64;
            }
        }


        o.v_o_num_str_value = o.a_o_num_str_value.iter().find(|&e| e.n == n_res).cloned();

        // println!("n_res {:#032b}", n_res);
        // println!("{} max {} o.v_o_input_sensor_value {:?} {}", o.s_name, n_value_max, o.v_o_input_sensor_value, o.n_nor);
        // println!("{: >40} {}", o.s_name, o.n_nor);

        n_idx_bit_start += n_bits as usize;
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



pub fn f_n_value(
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

// println!("{:?}", o_input_sensor);