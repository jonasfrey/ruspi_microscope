mod classes;
mod functions;

use std::thread;
use std::time::Duration;
use classes::O_input_sensor_value;
use functions::f_o_mutex_arc_o_stepper_28BYJ_48;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {



    let o_instant = std::time::Instant::now();
    let n_rpm_max = 10.;

    let (o_mutex_arc_clone, o_thread_handle) = f_o_mutex_arc_o_stepper_28BYJ_48([2,3,4,17]);
    // to update the stepper we have to create a scope
    {
        let mut o_stepper_28BYJ_48_x = o_mutex_arc_clone.lock().unwrap();
    
        // we can set properties of the stepper and then update it
        o_stepper_28BYJ_48_x.b_direction = true; 
        o_stepper_28BYJ_48_x.n_rpm_nor = 0.5;// normalized rpm
    }
    thread::sleep(Duration::from_millis(
        1000
    ));   
    {
        let mut o_stepper_28BYJ_48_x = o_mutex_arc_clone.lock().unwrap();
    
        o_stepper_28BYJ_48_x.n_rpm_nor = 0.01;// normalized rpm
    }
    thread::sleep(Duration::from_millis(
        1000
    ));  
    {
        let mut o_stepper_28BYJ_48_x = o_mutex_arc_clone.lock().unwrap();
    
        o_stepper_28BYJ_48_x.n_rpm_nor = 0.2;// normalized rpm
    }
    thread::sleep(Duration::from_millis(
        1000
    ));   
    {
        let mut o_stepper_28BYJ_48_x = o_mutex_arc_clone.lock().unwrap();
    
        o_stepper_28BYJ_48_x.n_rpm_nor = 0.0;// normalized rpm
    }
    thread::sleep(Duration::from_millis(
        1000
    ));  
    {
        let mut o_stepper_28BYJ_48_x = o_mutex_arc_clone.lock().unwrap();
    
        o_stepper_28BYJ_48_x.n_rpm_nor = 0.5;// normalized rpm
    }
    thread::sleep(Duration::from_millis(
        1000
    ));   
    // change direction
    {
        let mut o_stepper_28BYJ_48_x = o_mutex_arc_clone.lock().unwrap();
    
        o_stepper_28BYJ_48_x.b_direction = !o_stepper_28BYJ_48_x.b_direction;// normalized rpm
    }
    thread::sleep(Duration::from_millis(
        1000
    ));

    {
        let mut o_stepper_28BYJ_48_x = o_mutex_arc_clone.lock().unwrap();
    
        o_stepper_28BYJ_48_x.b_direction = !o_stepper_28BYJ_48_x.b_direction;// normalized rpm
    }
    thread::sleep(Duration::from_millis(
        1000
    ));
     
    thread::sleep(Duration::from_millis(
        180*1000
    ));   

    Ok(())
}
