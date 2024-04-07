
class O_ws_client{
    constructor(
        s_uuidv4,
        o_socket
    ){
        this.s_uuidv4 = s_uuidv4
        this.o_socket = o_socket
    }
}
// Define your classes
class o_num_str_value {
    constructor(n, s, s_comment) {
        this.n = n;
        this.s = s;
        this.s_comment = s_comment;
    }
}

class O_input_sensor {
    constructor(s_type, s_name, a_o_num_str_value = null) {
        this.s_type = s_type;
        this.s_name = s_name;
        this.a_o_num_str_value = a_o_num_str_value;
        this.value = 0;
        this.o_num_str_value = null;
        this.n_nor = 0;
    }
}
class O_usb_device_type{
    constructor(
        s_name, 
        s_emoji_icon
    ){
        this.s_name = s_name
        this.s_emoji_icon = s_emoji_icon
    }
}
class O_input_device {
    constructor(
        s_name,
         n_id_vendor,
         n_id_product,
         o_usb_device_type,
         a_o_input_sensor
        ) {
        this.s_name = s_name;
        this.n_id_vendor = n_id_vendor;
        this.n_id_product = n_id_product;
        this.o_usb_device_type = o_usb_device_type
        this.a_o_input_sensor = a_o_input_sensor;
    }
}

let f_s_first_letter_uppercase = function(s){
    return s.charAt(0).toUpperCase() + s.slice(1);
}
function f_n_from_string(s) {
    return parseInt(s.replace(/\D/g, ''), 10);
}

let f_update_o_input_device = function(
    o_input_device,
    a_n_u8
){
    let n_bit = 0
    let o_data_view = new DataView(a_n_u8.buffer);
    for (let n_idx in o_input_device.a_o_input_sensor){
        let o = o_input_device.a_o_input_sensor[n_idx]
        let n_idx_byte = parseInt(n_bit / 8) //# eg. 2
        let n_bits = f_n_from_string(o.s_type) //# for example 'u4' -> 4

        let s_name_function = [
            'get', 
            f_s_first_letter_uppercase(`${(o.s_type.includes('u') ? 'u': '')}int`),
            (Math.ceil(n_bits/8)*8).toString()
        ].join('');
        let b_little_endian = true;
        let n_value_number = o_data_view[
            s_name_function
        ](n_idx_byte, b_little_endian); 
        let n_idx_bit = n_bit % 8 //# eg. 4

        // console.log(
        //     `${n_idx_byte} ${s_name_function} ${n_value_number} ${n_idx_bit}`
        // )    
        let n_value_max = (Math.pow(2,n_bits)-1) //# eg. 2^4-1 = 16-1 = 15 => 0b1111
        if([8,16,32,64].includes(n_bits) == false){
            // todo , improve
            n_value_number = n_value_number >> (n_idx_bit) & n_value_max
        }
        if(o.s_type.includes('i')){
            n_value_max  = n_value_max /2
            //# n_value_byte -= n_value_max 
            // }
        } 

        o.value = n_value_number
        o.n_nor = n_value_number / n_value_max
        if(o.a_o_num_str_value){
            // console.log(o.a_o_num_str_value)
            o.o_enum_value = o.a_o_num_str_value.find(
                o=>{
                    // console.log(o)
                    return o.n == n_value_number
                }
            )
        }
        n_bit+=n_bits
        let v = o.n_nor
        if(o.o_enum_value){
            v = o.o_enum_value.s
        }
        // console.log(
        //     `${o.s_name.toString().padStart(30, ' ')}: ${v}`
        // )
    }
}