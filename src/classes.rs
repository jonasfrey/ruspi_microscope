use std::time::Instant;
use rppal::gpio::{Gpio, OutputPin};
use std::error::Error;

#[derive(Debug)]
#[derive(Clone)]
pub struct O_num_str_value {
    pub n: u64, 
    pub s: &'static str, 
    pub s_comment: &'static str, 
}

#[derive(Debug)]
#[derive(Clone)]
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

#[derive(Debug)]
#[derive(Clone)]
pub struct O_input_sensor {
    pub s_type: &'static str,
    pub s_name: &'static str,
    pub a_o_num_str_value: Option<Vec<O_num_str_value>>, 
    pub o_input_sensor_value: Option<O_input_sensor_value>,
    pub o_num_str_value: Option<O_num_str_value>, 
    pub n_nor: f64,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct O_input_device {
    pub s_name: &'static str,
    pub n_id_vendor: u16,
    pub n_id_product: u16,
    pub a_o_input_sensor: Vec<O_input_sensor>,
}





#[derive(Debug)]
pub struct O_stepper_28BYJ_48 {
    pub a_o_pin: Vec<OutputPin>,
    pub n_rpm_max: f64,
    pub n_rpm_nor: f64, 
    pub b_direction: bool, 
    pub n_substeps: u64, 
    pub n_radians: f64, 
    pub n_fullsteps_per_round: u32, 
    pub n_substeps_per_step: u32, 
    pub n_idx_a_o_pin: u8, 
    pub n_micsec_sleep_between_fullstep: f64, 
    pub n_micsec_ts_last_step : u128,
    pub o_instant: Instant
}