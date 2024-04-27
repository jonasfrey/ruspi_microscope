mod classes;
mod functions;

use classes::O_input_sensor_value;
use functions::f_o_mutex_arc_o_stepper_28BYJ_48;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {



    let o_instant = std::time::Instant::now();
    let n_rpm_max = 10.;

    let (o_mutex_arc_clone, o_thread_handle) = f_o_mutex_arc_o_stepper_28BYJ_48([2,3,4,17]);
    let mut o_stepper_28BYJ_48_x = o_mutex_arc_clone.lock().unwrap();

    // we can set properties of the stepper and then update it
    o_stepper_28BYJ_48_x.b_direction = true; 
    o_stepper_28BYJ_48_x.n_rpm_nor = 0.5;// normalized rpm

    let n_micsec_sleep_probe = 100. as f64;
    let mut n_micsec_last: u128 = o_instant.elapsed().as_micros();
    
    // loop{
    //     // println!("probe");
    //     let n_micsec_now = o_instant.elapsed().as_micros();
    //     let n_micsec_delta = (n_micsec_now - n_micsec_last) as f64;
    //     // println!("micsec delta {}", n_micsec_delta);

    //     // Perform the interrupt read, which would take around 8000 microsecs so we run a thread for it 
    //     // let n_b_read = o_device_handle.read_interrupt(0x81, &mut a_n_u8_read, o_duration__timeout)?;
    //     // f_update_o_input_device(&mut o_input_device, &a_n_u8_read);
    //     let o_input_sensor__right_x_axis =  f_o_input_sensor_from_s_name(&o_input_device.a_o_input_sensor, "right_x_axis" ).unwrap();
    //     let o_input_sensor__right_y_axis =  f_o_input_sensor_from_s_name(&o_input_device.a_o_input_sensor, "right_y_axis" ).unwrap();
    //     let o_input_sensor__r1 = f_o_input_sensor_from_s_name(&o_input_device.a_o_input_sensor, "r1" ).unwrap();
    //     let n_factor = if(o_input_sensor__r1.n_nor == 1.){ 0.1}else{1.};
        
    //     o_stepper_28BYJ_48_y.b_direction = o_input_sensor__right_y_axis.n_nor > 0.; 
    //     o_stepper_28BYJ_48_y.n_rpm_nor = o_input_sensor__right_y_axis.n_nor*n_factor;
    //     o_stepper_28BYJ_48_x.b_direction = o_input_sensor__right_x_axis.n_nor > 0.; 
    //     o_stepper_28BYJ_48_x.n_rpm_nor = o_input_sensor__right_x_axis.n_nor*n_factor;

    //     // println!("{:?}", o_input_device);
    //     // println!("right y axis{}", o_input_sensor__right_y_axis.n_nor);
    //     f_check_mic_sec_delta_and_potentially_step(&mut o_stepper_28BYJ_48_y);
    //     f_check_mic_sec_delta_and_potentially_step(&mut o_stepper_28BYJ_48_x);

    //     // while let Some(a_n_u8_read) = usb_read_receiver.await.recv().await {
    //     //     // Process the data received from the USB device
    //     //     println!("Received USB data: {:?}", a_n_u8_read);
    //     //     // Further processing...
    //     // }

    //     // match usb_read_receiver.await.try_recv() {
    //     //     Ok(a_n_u8_read) => {
    //     //         // Process the data received from the USB device
    //     //         // println!("Received USB data: {:?}", a_n_u8_read);
            
                
    //     //         f_update_o_input_device(&mut o_input_device, &a_n_u8_read);
    //     //         let o_input_sensor__d_pad_left = f_o_input_sensor_from_s_name(&o_input_device.a_o_input_sensor, "d_pad_left").unwrap();
    //     //         let o_input_sensor__d_pad_right = f_o_input_sensor_from_s_name(&o_input_device.a_o_input_sensor, "d_pad_right").unwrap();

    //     //         if(o_input_sensor__d_pad_right.n_nor == 1. && o_input_sensor__d_pad_right.n_nor__last != 1.){
    //     //             o_stepper_28BYJ_48_x.b_direction = true; 
    //     //             f_substep_o_stepper(&mut o_stepper_28BYJ_48_x)
    //     //         }
    //     //         if(o_input_sensor__d_pad_left.n_nor == 1. && o_input_sensor__d_pad_left.n_nor__last != 1.){
    //     //             o_stepper_28BYJ_48_x.b_direction = false; 
    //     //             f_substep_o_stepper(&mut o_stepper_28BYJ_48_x)
    //     //         }
    //     //     }
    //     //     Err(tokio::sync::mpsc::error::TryRecvError::Empty) => {
    //     //         // No data available yet; the main thread can perform other work
    //     //     }
    //     //     Err(e) => {
    //     //         // Handle other kinds of errors (e.g., the sender was disconnected)
    //     //         // eprintln!("Channel receive error: {:?}", e);
    //     //         break;
    //     //     }
    //     // }

    //     let n_micsec_probe_diff = n_micsec_sleep_probe - n_micsec_delta;
    //     // println!("probe sleep {}", n_micsec_probe_diff);
    //     if(n_micsec_probe_diff > 0.){
    //         thread::sleep(Duration::from_micros(
    //             (n_micsec_probe_diff as u128).try_into().unwrap()
    //         ));   
    //     }
    //     n_micsec_last = n_micsec_now;

    // }




    Ok(())
}
