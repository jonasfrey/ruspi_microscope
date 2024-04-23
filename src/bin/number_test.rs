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
    println!("min|range|val|val+abs(min)|nor {}|{}|{}|{}|{}", min_value.abs(),n_range, value, value + min_value.abs(), normalized);
    normalized
}


fn main() {
    let data: Vec<u8> = vec![0, 255,236, 233];
    let start_bit = 10;
    let bit_count = 12;
    let signed = true;
    let bigendian = true;

    let value = f_n_extracted(&data, start_bit, bit_count, signed, bigendian);
    let normalized = f_n_normalized(value as i128, bit_count as u8, signed);

    println!("Extracted value: {}", value);
    println!("Normalized number: {}", normalized);
}
