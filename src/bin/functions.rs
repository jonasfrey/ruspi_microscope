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

// println!("{:?}", o_input_sensor);