
use sysinfo::{ProcessExt, System, SystemExt};
use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;
use std::env;
use nix::libc;

fn kill_existing_instances(process_name: &str) -> bool {
    let mut system = System::new_all();
    system.refresh_all();

    let current_pid = std::process::id();
    let mut process_killed = false;

    // Iterate through all processes
    for (pid, process) in system.processes() {
        if process.name() == process_name && *pid as u32 != current_pid {
            println!("Found existing instance with PID {}. Killing it.", pid);
            // Kill the process
            match kill(Pid::from_raw(*pid as i32), Signal::SIGINT) {
                Ok(_) => {
                    println!("Successfully sent SIGINT to process {}", pid);
                    process_killed = true;
                },
                Err(e) => println!("Failed to send SIGINT to process {}: {}", pid, e),
            }
        }
    }

    process_killed
}
fn set_process_name(name: &str) {
    let cname = std::ffi::CString::new(name).unwrap();
    unsafe {
        libc::prctl(libc::PR_SET_NAME, cname.as_ptr(), 0, 0, 0);
    }
}


fn main(){
    let current_exe = env::current_exe().unwrap();
    let process_name = current_exe.file_name().unwrap().to_str().unwrap();
    
    // Set the process name (optional, depending on the system)
    set_process_name(process_name);

    if kill_existing_instances(process_name) {
        println!("An existing instance was killed.");
    }
    println!("programm running");
    loop{
        
    }

}
