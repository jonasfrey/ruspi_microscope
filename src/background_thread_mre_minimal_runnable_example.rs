use rppal::gpio::Gpio;
use std::{error::Error, os::unix::process, process::exit, sync::{Arc, Mutex}, thread::{self, JoinHandle}, time::{Duration, SystemTime, UNIX_EPOCH}};
use rusb::{Device, UsbContext, open_device_with_vid_pid};
use core::task::Context;


fn f_spawn_thread(o_test_arc: Arc<Mutex<O_test>>) -> JoinHandle<()> {
    thread::spawn( move || 
        // {
        //     // This loop will run in the background
        //     let mut n = 0;
        //     loop {
        //         n+=1;
        //         let o_test = o_test_arc.lock().unwrap();
        //         println!("Background loop iteration: {} o_test.n is {}", n, o_test.n);
        //         // Sleep for a bit to simulate work and make output readable
        //         thread::sleep(Duration::from_millis(200));
        //     }
        // }
        loop {
            {
                // Lock the mutex to safely access the shared state
                let mut o_test = o_test_arc.lock().unwrap();
                println!("o_test.n is {}",  o_test.n);

            } // MutexGuard is dropped here, releasing the lock
    
            // Sleep to prevent the loop from consuming too much CPU
            thread::sleep(Duration::from_secs(1));
        }
)

}
#[derive(Debug)]
#[derive(Clone)]
pub struct O_test {
    pub n :u8,
}
fn main() -> Result<(), Box<dyn Error>> {

    let o_test = O_test{n: 0};
    let o_test_arc = Arc::new(Mutex::new(o_test));
    let o_join_handle = f_spawn_thread(Arc::clone(&o_test_arc));

    
    thread::sleep(Duration::from_millis(2000));
    {
        let mut o_test = o_test_arc.lock().unwrap();
        o_test.n = 1; // bigger rpm
    }
    thread::sleep(Duration::from_millis(2000));
    {
        let mut o_test = o_test_arc.lock().unwrap();
        o_test.n = 18; // bigger rpm
    }
    thread::sleep(Duration::from_millis(2000));
    {
        let mut o_test = o_test_arc.lock().unwrap();
        o_test.n = 200; // bigger rpm
    }
    println!("are we here?");
    let _ = o_join_handle.join();
    Ok(())
}
