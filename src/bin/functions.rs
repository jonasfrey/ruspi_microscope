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

fn f_n_extracted(
    data: &[u8],
    start_bit: usize,
    bit_count: usize,
    signed: bool,
    bigendian: bool,
) -> i64 {
    let mut value: i64 = 0;
    let mut bit_position = start_bit;

    // Extract the specified number of bits from the data array
    for _ in 0..bit_count {
        let byte_index = bit_position / 8;
        let bit_offset = bit_position % 8;

        if byte_index >= data.len() {
            break; // Prevent overflow
        }

        // Get the bit value at the specified position
        let bit_value = if bigendian {
            (data[byte_index] >> (7 - bit_offset)) & 1
        } else {
            (data[byte_index] >> bit_offset) & 1
        };

        value = (value << 1) | bit_value as i64;

        bit_position += 1;
    }

    if signed {
        let max_unsigned_value = 1 << (bit_count - 1);
        // Convert the extracted value to signed if necessary
        if value >= max_unsigned_value {
            value -= (1 << bit_count);
        }
    }

    value
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
        let mut n_res = f_n_extracted(
            a_n_u8,
            n_idx_bit_start, 
            n_bits, 
            b_signed, 
            true
        );
        let n_bits_rounded_up = ((n_bits / 8) as f32).ceil() * 8.0;
        // println!("bit index start:end {}:{}", n_idx_bit_start, n_idx_bit_end);
        let mut n_value_max = (1u64 << n_bits) - 1;
        if(b_signed){
            n_value_max -=1;
        }
        o.n_nor__last = o.n_nor;

        if(n_bits <= 8){
            if(b_signed){
                o.v_o_input_sensor_value = Some(O_input_sensor_value::I8(n_res as i8));
            }
            if(b_unsigned){
                o.v_o_input_sensor_value = Some(O_input_sensor_value::U8(n_res as u8));
            }
        }
        if(n_bits > 8 && n_bits <= 16){

            if(b_signed){
                o.v_o_input_sensor_value = Some(O_input_sensor_value::I16(n_res as i16));
            }
            if(b_unsigned){
                o.v_o_input_sensor_value = Some(O_input_sensor_value::U16(n_res as u16));
            }
        }
        if(n_bits > 16 && n_bits <= 32){
            if(b_float){
                o.v_o_input_sensor_value = Some(O_input_sensor_value::F32(n_res as f32));
            }
            if(b_signed){
                o.v_o_input_sensor_value = Some(O_input_sensor_value::I32(n_res as i32));
            }
            if(b_unsigned){
                o.v_o_input_sensor_value = Some(O_input_sensor_value::U32(n_res as u32));
            }
        }
        if(n_bits > 32 && n_bits <= 64){
            if(b_float){
                o.v_o_input_sensor_value = Some(O_input_sensor_value::F64(n_res as f64));
            }
            if(b_signed){
                o.v_o_input_sensor_value = Some(O_input_sensor_value::I64(n_res as i64));
            }
            if(b_unsigned){
                o.v_o_input_sensor_value = Some(O_input_sensor_value::U64(n_res as u64));
            }
        }
        
        // print!("\x1B[2J\x1B[H"); // Clear the screen and move the cursor to the top-left
        // std::io::stdout().flush().unwrap();
        println!("{}", n_res);

        o.n_nor = f_n_normalized(n_res as i128, n_bits as u8, b_signed);
        // we treat nor as normalized and thus always from 0.0 to 1.0, 
        // if the number is signed it would be 'snor'
        // this is because in gamepad usb controllers for example 
        // not all controllers use a signed number for stick axis, then the value is 0.5 in the middle 
        // to make it easier we only provide a normalized value, the programmer then can get the information
        // if a value is signed or not by looking at the 's_type' which can be 'u8' or 'i8' etc.
        // if(b_signed){
        //     o.n_nor +=0.5;
        // }

        o.v_o_num_str_value = o.a_o_num_str_value.iter().find(|&e| e.n == n_res as u64).cloned();

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