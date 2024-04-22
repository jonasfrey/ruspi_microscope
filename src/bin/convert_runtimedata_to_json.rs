pub mod classes; 
use image::load_from_memory;
pub mod runtimedata; 
use runtimedata::f_a_o_input_device;
use std::{fs::File, io::Write};
// Main function or wherever you need to perform serialization
fn main(){
    let devices = f_a_o_input_device(); // Assuming this returns Vec<OInputDevice>
    let json = serde_json::to_string_pretty(&devices).unwrap();
    println!("{}", json);

    let mut file = File::create("./json.json").expect("cannot create file");
    file.write_all(json.as_bytes()).expect("cannot write file");

}