use std::process::Command;
use std::io::{self, Write};
use std::env;


fn main() -> io::Result<()>{

    // install denojs


    let status = Command::new("sh")
    .arg("-c")
    .arg("curl -fsSL https://deno.land/install.sh | sh")
    .status()
    .expect("Failed to execute install script");
 
     if !status.success() {
     eprintln!("Installation script failed");
     return Err(io::Error::new(io::ErrorKind::Other, "Installation failed"));
     }
 
     // Add Deno to the PATH environment variable
     let home_dir = env::var("HOME").expect("Failed to get HOME directory");
     let deno_bin_path = format!("{}/.deno/bin", home_dir);
     let current_path = env::var("PATH").expect("Failed to get PATH environment variable");
 
     if !current_path.contains(&deno_bin_path) {
     let new_path = format!("{}:{}", deno_bin_path, current_path);
     env::set_var("PATH", new_path);
     println!("Deno binary added to PATH.");
     } else {
     println!("Deno binary is already in PATH.");
     }
 
     // Verify the installation
     let output = Command::new("deno")
     .arg("--version")
     .output()
     .expect("Failed to execute Deno");
 
     if output.status.success() {
     println!("Deno installed successfully:");
     io::stdout().write_all(&output.stdout)?;
     } else {
     println!("Failed to install Deno:");
     io::stderr().write_all(&output.stderr)?;
     }


    let mut o_command = Command::new("deno");
    o_command.arg("run");
    o_command.arg("-A");
    o_command.arg("./autogenerate.js");

    println!("Executing command: {:?}", o_command);
    let o_output = o_command.output().unwrap();
    println!("Output: {:?}", String::from_utf8_lossy(&o_output.stdout));
    println!("Error: {:?}", String::from_utf8_lossy(&o_output.stderr));

    Ok(())
}