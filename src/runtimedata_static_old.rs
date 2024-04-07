use crate::classes::{
    O_input_device,
    O_input_sensor,
    O_input_sensor_value,
    O_num_str_value
};


pub static o_input_device__shenzhen_shanwan_android_gamepad : O_input_device = O_input_device {
    s_name: "ShenZhen ShanWan Technology Co., Ltd. Android Gamepad",
    n_id_vendor: 9571,
    n_id_product: 1318,
    a_o_input_sensor: vec![
        O_input_sensor {
            s_type: "u8",
            s_name: "padding_or_not_found_out_yet",
            a_o_num_str_value: None,
            value: None,
            o_num_str_value: None,
            n_nor: 0.0,     
        },
        O_input_sensor {
            s_type: "u8",
            s_name: "left_x_axis",
            a_o_num_str_value: None,
            value: Some(O_input_sensor_value::U8(0)),
            o_num_str_value: None,
            n_nor: 0.0,
        },
        O_input_sensor {
            s_type: "u8",
            s_name: "left_y_axis",
            a_o_num_str_value: None,
            value: Some(O_input_sensor_value::U8(0)),
            o_num_str_value: None,
            n_nor: 0.0,
        },
        O_input_sensor {
            s_type: "u8",
            s_name: "right_x_axis",
            a_o_num_str_value: None,
            value: Some(O_input_sensor_value::U8(0)),
            o_num_str_value: None,
            n_nor: 0.0,
        },
        O_input_sensor {
            s_type: "u8",
            s_name: "right_y_axis",
            a_o_num_str_value: None,
            value: Some(O_input_sensor_value::U8(0)),
            o_num_str_value: None,
            n_nor: 0.0,
        },
        O_input_sensor {
            s_type: "u8",
            s_name: "d_pad_values",
            a_o_num_str_value: Some(vec![
                O_num_str_value { n: 0, s: "up", s_comment: "D-Pad Up" },
                O_num_str_value { n: 1, s: "up_right", s_comment: "D-Pad Up Right" },
                O_num_str_value { n: 2, s: "right", s_comment: "D-Pad Right" },
                O_num_str_value { n: 3, s: "right_down", s_comment: "D-Pad Right Down" },
                O_num_str_value { n: 4, s: "down", s_comment: "D-Pad Down" },
                O_num_str_value { n: 5, s: "down_left", s_comment: "D-Pad Down Left" },
                O_num_str_value { n: 6, s: "left", s_comment: "D-Pad Left" },
                O_num_str_value { n: 7, s: "left_up", s_comment: "D-Pad Left Up" },
                O_num_str_value { n: 8, s: "none", s_comment: "D-Pad None" },
            ]),
            value: None,
            o_num_str_value: None,
            n_nor: 0.0,
        },
        O_input_sensor {
            s_type: "u1",
            s_name: "A",
            a_o_num_str_value: None,
            value: Some(O_input_sensor_value::Bool(true)),
            o_num_str_value: None,
            n_nor: 0.0,
        },
        O_input_sensor {
            s_type: "u1",
            s_name: "B",
            a_o_num_str_value: None,
            value: Some(O_input_sensor_value::Bool(false)),
            o_num_str_value: None,
            n_nor: 0.0,
        },
        // Repeat for 'X', 'Y', 'L1', 'R1', 'L2_pressed', 'R2_pressed', 'back_select', 'Start', 'L3', 'R3'
        O_input_sensor {
            s_type: "u1",
            s_name: "X",
            a_o_num_str_value: None,
            value: Some(O_input_sensor_value::Bool(false)),
            o_num_str_value: None,
            n_nor: 0.0,
        },
        O_input_sensor {
            s_type: "u1",
            s_name: "Y",
            a_o_num_str_value: None,
            value: Some(O_input_sensor_value::Bool(false)),
            o_num_str_value: None,
            n_nor: 0.0,
        },
        // Assuming placeholders for 'L1', 'R1', 'L2_pressed', 'R2_pressed', 'back_select', 'Start', 'L3', 'R3'
        O_input_sensor {
            s_type: "u8",
            s_name: "R2_intensity",
            a_o_num_str_value: None,
            value: Some(O_input_sensor_value::U8(0)), // Placeholder value
            o_num_str_value: None,
            n_nor: 0.0,
        },
        O_input_sensor {
            s_type: "u8",
            s_name: "L2_intensity",
            a_o_num_str_value: None,
            value: Some(O_input_sensor_value::U8(0)), // Placeholder value
            o_num_str_value: None,
            n_nor: 0.0,
        },
    ],
};

pub static o_input_device__xbox360_controller : O_input_device = O_input_device {
    s_name: "Microsoft Corp. Xbox360 Controller",
    n_id_vendor: 0x045e,
    n_id_product: 0x028e,
    a_o_input_sensor: vec![
        O_input_sensor {
            s_type: "u8",
            s_name: "padding_or_not_found_out_yet",
            a_o_num_str_value: None,
            value: None,
            o_num_str_value: None,
            n_nor: 0.0,
        },
        O_input_sensor {
            s_type: "u8",
            s_name: "padding_or_not_found_out_yet",
            a_o_num_str_value: None,
            value: None,
            o_num_str_value: None,
            n_nor: 0.0,
        },
        O_input_sensor {
            s_type: "u1",
            s_name: "d_pad_up",
            a_o_num_str_value: None,
            value: Some(O_input_sensor_value::Bool(false)), // Assuming initial state is not pressed
            o_num_str_value: None,
            n_nor: 0.0,
        },
        O_input_sensor {
            s_type: "u1",
            s_name: "d_pad_down",
            a_o_num_str_value: None,
            value: Some(O_input_sensor_value::Bool(false)),
            o_num_str_value: None,
            n_nor: 0.0,
        },
        O_input_sensor {
            s_type: "u1",
            s_name: "d_pad_left",
            a_o_num_str_value: None,
            value: Some(O_input_sensor_value::Bool(false)),
            o_num_str_value: None,
            n_nor: 0.0,
        },
        O_input_sensor {
            s_type: "u1",
            s_name: "d_pad_right",
            a_o_num_str_value: None,
            value: Some(O_input_sensor_value::Bool(false)),
            o_num_str_value: None,
            n_nor: 0.0,
        },
        O_input_sensor {
            s_type: "u1",
            s_name: "start",
            a_o_num_str_value: None,
            value: Some(O_input_sensor_value::Bool(false)),
            o_num_str_value: None,
            n_nor: 0.0,
        },
        O_input_sensor {
            s_type: "u1",
            s_name: "select",
            a_o_num_str_value: None,
            value: Some(O_input_sensor_value::Bool(false)),
            o_num_str_value: None,
            n_nor: 0.0,
        },
        // Additional buttons follow the same pattern
        O_input_sensor {
            s_type: "u1",
            s_name: "l3",
            a_o_num_str_value: None,
            value: Some(O_input_sensor_value::Bool(false)),
            o_num_str_value: None,
            n_nor: 0.0,
        },
        O_input_sensor {
            s_type: "u1",
            s_name: "r3",
            a_o_num_str_value: None,
            value: Some(O_input_sensor_value::Bool(false)),
            o_num_str_value: None,
            n_nor: 0.0,
        },
        O_input_sensor {
            s_type: "u1",
            s_name: "l1",
            a_o_num_str_value: None,
            value: Some(O_input_sensor_value::Bool(false)),
            o_num_str_value: None,
            n_nor: 0.0,
        },
        O_input_sensor {
            s_type: "u1",
            s_name: "r1",
            a_o_num_str_value: None,
            value: Some(O_input_sensor_value::Bool(false)),
            o_num_str_value: None,
            n_nor: 0.0,
        },
        O_input_sensor {
            s_type: "u1",
            s_name: "padding_or_not_found_out_yet",
            a_o_num_str_value: None,
            value: Some(O_input_sensor_value::Bool(false)),
            o_num_str_value: None,
            n_nor: 0.0,
        },
        O_input_sensor {
            s_type: "u1",
            s_name: "analog_button",
            a_o_num_str_value: None,
            value: Some(O_input_sensor_value::Bool(false)),
            o_num_str_value: None,
            n_nor: 0.0,
        },
        O_input_sensor {
            s_type: "u1",
            s_name: "x_button",
            a_o_num_str_value: None,
            value: Some(O_input_sensor_value::Bool(false)),
            o_num_str_value: None,
            n_nor: 0.0,
        },
        O_input_sensor {
            s_type: "u1",
            s_name: "o_button",
            a_o_num_str_value: None,
            value: Some(O_input_sensor_value::Bool(false)),
            o_num_str_value: None,
            n_nor: 0.0,
        },
        O_input_sensor {
            s_type: "u1",
            s_name: "square_button",
            a_o_num_str_value: None,
            value: Some(O_input_sensor_value::Bool(false)),
            o_num_str_value: None,
            n_nor: 0.0,
        },
        O_input_sensor {
            s_type: "u1",
            s_name: "triangle_button",
            a_o_num_str_value: None,
            value: Some(O_input_sensor_value::Bool(false)),
            o_num_str_value: None,
            n_nor: 0.0,
        },
        // Omitting for brevity, repeat for each sensor as needed
        O_input_sensor {
            s_type: "u8",
            s_name: "l2_intensity",
            a_o_num_str_value: None,
            value: Some(O_input_sensor_value::U8(0)), // Placeholder
            o_num_str_value: None,
            n_nor: 0.0,
        },
        O_input_sensor {
            s_type: "u8",
            s_name: "r2_intensity",
            a_o_num_str_value: None,
            value: Some(O_input_sensor_value::U8(0)), // Placeholder
            o_num_str_value: None,
            n_nor: 0.0,
        },
        O_input_sensor {
            s_type: "i16",
            s_name: "left_x_axis",
            a_o_num_str_value: None,
            value: Some(O_input_sensor_value::I16(0)), // Placeholder for the axis value
            o_num_str_value: None,
            n_nor: 0.0,
        },
        O_input_sensor {
            s_type: "i16",
            s_name: "left_y_axis",
            a_o_num_str_value: None,
            value: Some(O_input_sensor_value::I16(0)), // Placeholder for the axis value
            o_num_str_value: None,
            n_nor: 0.0,
        },
        O_input_sensor {
            s_type: "i16",
            s_name: "right_x_axis",
            a_o_num_str_value: None,
            value: Some(O_input_sensor_value::I16(0)), // Placeholder for the axis value
            o_num_str_value: None,
            n_nor: 0.0,
        },
        O_input_sensor {
            s_type: "i16",
            s_name: "right_y_axis",
            a_o_num_str_value: None,
            value: Some(O_input_sensor_value::I16(0)), // Placeholder for the axis value
            o_num_str_value: None,
            n_nor: 0.0,
        },
    ],
};


// println!("{:?}", o_input_sensor);