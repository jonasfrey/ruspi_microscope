use std::time::Instant;
use rppal::gpio::{Gpio, OutputPin};
use std::error::Error;
use serde::{forward_to_deserialize_any, Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct O_num_str_value {
    pub n: u64, 
    pub a_s_name: Vec<String>, 
    pub s_comment: String, 
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum O_input_sensor_value {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    F32(f32),
    F64(f64),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct O_input_sensor {
    pub s_type: String,
    pub s_name: String,
    pub v_o_input_sensor_value: Option<O_input_sensor_value>,
    pub a_o_num_str_value: Vec<O_num_str_value>, 
    pub v_o_num_str_value: Option<O_num_str_value>, 
    pub n_nor: f64,
    pub n_nor__last: f64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct O_name_synonym{
    pub s_name: String,
    pub a_s_synonym: Vec<String>,
} 
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct O_input_device {
    pub s_name: String,
    pub n_id_vendor: u16,
    pub n_id_product: u16,
    pub a_o_input_sensor: Vec<O_input_sensor>,
}


#[derive(Debug)]
pub struct O_stepper_28BYJ_48 {
    pub a_o_pin: Vec<OutputPin>,
    pub b_depower_if_rpm_zero: bool,
    pub n_rpm_max: f64,
    pub n_rpm_nor: f64, 
    pub n_rpm_nor_last: f64, 
    pub b_direction: bool, 
    pub n_substeps: u64, 
    pub n_idx_substep: u8,
    pub n_radians: f64, 
    pub n_fullsteps_per_round: u32, 
    pub n_substeps_per_step: u32, 
    pub n_micsec_sleep_between_fullstep: f64, 
    pub n_micsec_ts_last_step : u128,
    pub o_instant: Instant
}

pub type A_o_input_device = Vec<O_input_device>;
pub type A_o_name_synonym = Vec<O_name_synonym>;


pub enum ControlCommand {
    Start,
    Stop,
    SwitchDevice(u16, u16),  // Vendor ID, Product ID
}

#[derive(Clone)]
pub struct SendData{
    pub a_n_u8_usb_read_result: Option<Vec<u8>>,
    pub v_o_input_device: Option<O_input_device>
}
