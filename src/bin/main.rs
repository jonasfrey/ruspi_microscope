// disable warnings
#![allow(warnings)] 

use tokio::{
    sync::broadcast
};
use warp::{
    Filter, 
    ws::{Message, WebSocket}
};
use futures::{
    StreamExt, SinkExt
};
use dataurl::DataUrl;
use std::{
    path::{
        Path, 
        PathBuf
    },
    os::unix::fs::PermissionsExt,
    io::{
        self,
        ErrorKind,
        Write
    },
    fs::{
        self, 
        File
    },
    time::{
        SystemTime,
        UNIX_EPOCH,
        Duration
    },
    process::{Command}, 
    sync::{
        mpsc, Arc, Mutex
    },
    thread
};
use serde_json::{
    json, 
    Map, 
    Value
};

use rusb::{
    DeviceHandle, 
    GlobalContext,
    open_device_with_vid_pid, 
    Direction, 
    TransferType
};


use classes::A_o_input_device;
use classes::A_o_name_synonym;
pub mod classes;
pub mod functions;


use functions::{
    f_b_gpio_available,
    f_update_o_input_device,
    f_o_sender_tx_spawn_thread_with_event_listener_for_stepper,
    f_o_input_sensor_from_s_name, 
    f_ensure_directory,
    f_s_extension_from_s_mime_type,
    f_create_or_clean_directory,
    f_b_contains_image_file_suffix,
    f_b_directory_contains_more_than_one_image, 
    f_move_files, 
    f_install_denojs
};
use classes::{
    ControlCommand, O_input_device, SendData
};

use gethostname::gethostname;

const s_path_rel_file__config: &'static str = "./o_config.json";
const s_path_rel_folder__webroot: &'static str = "./public";
const s_path_rel_file__focus_stack_binary: &'static str = "./focus-stack.AppImage";
// const s_path_rel_file__image_stitching_binary: &'static str = "stitch"; // https://github.com/OpenStitching/stitching pip install stitching
const s_path_rel_file__image_stitching_binary: &'static str = "python3"; 

fn f_usb_read_thread(
    o_tx_control_usb: mpsc::Sender<SendData>,
    o_rx_control_usb: mpsc::Receiver<ControlCommand>, 
) {


    let s_json = fs::read_to_string("./o_info.json").expect("Unable to read file");
    // Deserialize the JSON data into a serde_json::Value
    let v: serde_json::Value = serde_json::from_str(&s_json).expect("cannot parse json");
    let mut a_o_input_device: A_o_input_device = serde_json::from_value(v.get("a_o_input_device").expect("json must have a_o_input_device").clone()).expect("cannot decode json");
    let mut a_o_name_synonym: A_o_name_synonym = serde_json::from_value(v.get("a_o_name_synonym").expect("json must have a_o_name_synonym").clone()).expect("cannot decode json");
    // println!("a_o_input_device {:?}", a_o_input_device);
    let mut v_o_input_device: Option<&mut O_input_device> = None;
    let mut o_device_handle_option: Option<DeviceHandle<GlobalContext>> = None;
    let mut b_usb_readout_active = false;
    let mut n_id_vendor = 0;
    let mut n_id_product = 0;
    let mut n_id_vendor_old = 0;
    let mut n_id_product_old = 0;
    let mut o_timeout = Duration::from_millis(1000);
    let mut n_len_a_n_u8__readout = 32;
    let mut n_interface = 0;
    let mut n_address_endpoint_in = 0x81;
    let n_class_code_human_interface_device = 3;
    loop {
        // Check for control commands
        match o_rx_control_usb.try_recv() {
            Ok(ControlCommand::Start) => {
                b_usb_readout_active = true;
            },
            Ok(ControlCommand::Stop) => {
                b_usb_readout_active = false;
            },
            Ok(ControlCommand::SwitchDevice(vid, pid)) => {
                if(o_device_handle_option.is_some()){
                    println!("release old iface and attach kernel driv");
                    if let Some(ref mut o_device_handle) = o_device_handle_option{
                        let _ = o_device_handle.release_interface(n_interface).expect("cannot release interface");
                        // let _ = o_device_handle.attach_kernel_driver(0).expect("cannot attach kernel driver");
                    }
                    // release the old interface and attach the kernel driver
                }
                
                n_id_vendor = vid;
                n_id_product = pid;
                v_o_input_device = a_o_input_device
                    .iter_mut()
                    .find(
                        |o| o.n_id_vendor == n_id_vendor && o.n_id_product == n_id_product
                    );

                o_device_handle_option = open_device_with_vid_pid(vid, pid);
                match o_device_handle_option{
                    Some(ref mut o_device_handle)=>{
                        b_usb_readout_active = true;

                        let o_command_old = Command::new("lsusb")
                                            .args(["-d", &format!("{:#02x}:{:#02x}", n_id_vendor_old, n_id_product_old)])
                                            .output()
                                            .expect("failed to execute process");
                        let o_command_new = Command::new("lsusb")
                                        .args(["-d", &format!("{:#02x}:{:#02x}", n_id_vendor, n_id_product)])
                                        .output()
                                        .expect("failed to execute process");
                        n_id_vendor_old = vid;
                        n_id_product_old = pid;
                        println!(
                            "switching usb device: 
                            old: {:?}
                            new: {}
                        ",
                        String::from_utf8_lossy(&o_command_old.stdout),
                        String::from_utf8_lossy(&o_command_new.stdout)
                        );
                        let o_config_descriptor = o_device_handle.device().active_config_descriptor().expect("Failed to get configuration descriptor");
                        let mut b_interface_found: bool = false;

                        for o_interface in o_config_descriptor.interfaces() {
                            for o_interface_descriptor in o_interface.descriptors() {
                                for o_endpoint_descriptor in o_interface_descriptor.endpoint_descriptors() {
                                    let n_dir = o_endpoint_descriptor.direction();
                                    let n_tt = o_endpoint_descriptor.transfer_type();
                                    let n_class = o_interface_descriptor.class_code();
                                    let n_ifacenum = o_interface.number();
                                    println!(
                                        "n_dir:{:?}
                                        n_tt:{:?}
                                        n_ifacenum:{:?}
                                        n_class:{:?}",
                                        n_dir,
                                        n_tt,
                                        n_ifacenum, 
                                        n_class
                                    );
                                    if n_dir == Direction::In
                                        && n_tt == TransferType::Interrupt
                                    {
                
                                        n_address_endpoint_in = o_endpoint_descriptor.address();
                                        let n = o_endpoint_descriptor.interval() as f32;
                                        n_interface = o_interface.number();
                                        o_timeout = Duration::from_millis((n*5.00) as u64);// account for overhead and ensure reliability
                                        n_len_a_n_u8__readout = o_endpoint_descriptor.max_packet_size();
                                        if(
                                            n_class == n_class_code_human_interface_device
                                        ){
                                            b_interface_found = true;
                                        }
                                        // some devices have 255 which is 0xff usb class code FFh Both Vendor Specific
                                            
                                    }
                                    if(b_interface_found){break;}
                                }
                                if(b_interface_found){break;}
                            }
                            if(b_interface_found){break;}
                        }
                        println!(
                            "found interrupt interface: 
                            {}, 
                            endpoint IN 
                            addr:{}, 
                            timeout:{:?}, 
                            n_len_a_n_u8_readout {:?}
                            ", n_interface,
                            n_address_endpoint_in,
                            o_timeout,
                            n_len_a_n_u8__readout
                        );


                        // if no suitable interface is found the 'default' values will be used
                            let _ = o_device_handle.set_auto_detach_kernel_driver(true).expect("cannot set auto a- de- tach of the kernel driver");
                            let _ = o_device_handle.claim_interface(n_interface).expect(
                                &format!("Cannot claim interface, {}", n_interface)
                            );
                    }
                    None =>{
                        println!("cannot open usb device, is it connected and are you root?");
                        b_usb_readout_active = false;
                    }
                }
                


            },
            Err(mpsc::TryRecvError::Empty) => {},
            Err(_) => break,  // Exit on other errors
        }

        if b_usb_readout_active {
            // println!("readout?");
            let mut a_n_u8__readout = vec![0u8; n_len_a_n_u8__readout as usize];
            if let Some(ref mut o_device_handle) = o_device_handle_option{
                // println!("readout?");
                match o_device_handle.read_interrupt(n_address_endpoint_in, &mut a_n_u8__readout, o_timeout) {
                    Ok(n_bytes_read) => {
                        let mut v_o_input_device_cloned = None;
                        if let Some(ref mut o_input_device) = v_o_input_device{
                            v_o_input_device_cloned = Some(o_input_device.clone()); 
                            f_update_o_input_device(
                                o_input_device, 
                                &a_n_u8__readout
                            );
                            // print!("\x1B[2J\x1B[H"); // Clear the screen and move the cursor to the top-left
                            // io::stdout().flush().unwrap();
                        
                            // for o_input_sensor in &o_input_device.a_o_input_sensor{

                            //     println!("{:?}:{:?} {:?}", o_input_sensor.s_name, o_input_sensor.n_nor, o_input_sensor.v_o_num_str_value);
                            // }
                        }
                        // println!("Read from USB device success, bytes read: {:?}", a_n_u8__readout);
                        o_tx_control_usb.send(SendData{
                            a_n_u8_usb_read_result: Some(a_n_u8__readout),
                            v_o_input_device: v_o_input_device_cloned
                        }).expect("Failed to send data to main thread");

                        // Send data to the main application if needed
                    },
                    Err(e) => {
                        eprintln!("USB read error: {:?}", e);
                    }
                }
            }

        }

        // Sleep to prevent tight looping
        thread::sleep(Duration::from_millis(10));
    }
}


fn f_b_write_s_text_file(s_str: &str, s_path_rel: &str) -> bool{
    let mut file = File::create(s_path_rel).expect("Failed to create file");

    file.write_all(s_str.as_bytes()).expect("Failed to write to file");


    fs::set_permissions(s_path_rel, fs::Permissions::from_mode(0o777)).unwrap();

    return true;
    // println!("Data written to {}", s_path_rel_file__config);
}

fn f_s_read_text_file(s_path_rel: &str) -> String {
    match fs::read_to_string(s_path_rel) {
        Ok(s_content) => s_content,
        Err(e) => {
            if let ErrorKind::NotFound = e.kind() {
                "".to_string() // Return an empty string if the file is not found
            } else {
                panic!("Failed to read file: {:?}", e) // Panic for other errors
            }
        }
    }
}

async fn f_websocket_thread(
    mut o_websocket: WebSocket,
    mut o_rx_receiver: broadcast::Receiver<String>, 
    mut o_tx_sender_clone: broadcast::Sender<String>, 
    o_tx_control_usb: mpsc::Sender<ControlCommand>, 
    v_o_sender_tx_stepper_28BYJ_48_x:Option<mpsc::Sender::<String>>,
    v_o_sender_tx_stepper_28BYJ_48_y:Option<mpsc::Sender::<String>>,
    v_o_sender_tx_stepper_28BYJ_48_z: Option<mpsc::Sender::<String>>
) { 
    let (mut o_ws_sender, mut o_ws_receiver) = o_websocket.split();

    // Task to forward broadcast messages to WebSocket client
    let forward_task = tokio::spawn(async move {
        while let Ok(s_msg) = o_rx_receiver.recv().await {
            let o_msg = Message::text(s_msg);
            if o_ws_sender.send(o_msg).await.is_err() {
                eprintln!("Failed to send WebSocket message");
                break;
            }
        }
    });

    // Optionally handle incoming messages
    while let Some(result) = o_ws_receiver.next().await {
        if let Ok(msg) = result {
            if msg.is_close() {
                break;
            }
            if let Ok(text) = msg.to_str() {
                println!("websocket received message");
                if let Ok(v_json_parsed) = serde_json::from_str::<serde_json::Value>(text) {

                    // s_name_function must be present as it is an identifier of what to do on the server
                    if let Some(s_name_function) = v_json_parsed.get("s_name_function").and_then(|v: &serde_json::Value| v.as_str()) {
                        // if 's_uuid' is present the response json should also contain it since the client expects it as a response
                        if let Some(s_uuid) = v_json_parsed.get("s_uuid").and_then(|v: &serde_json::Value| v.as_str()) {
                            let mut o_response = Map::new();                                
                                
                            if(s_name_function == "hello"){
                                o_response.insert("s_res".to_string(), json!("World !"));
                                o_response.insert("o_child".to_string(), json!({ "s_stdout__lsusb": "yes" }));
                            }


                            if(s_name_function == "f_save_screenshot"){

                                if let Some(s_data_url) = v_json_parsed.get("s_data_url").and_then(|v| v.as_str()) {

                                    if let Some(s_name_file) = v_json_parsed.get("s_name_file").and_then(|v| v.as_str()) {


                                        let s_config_json: String = fs::read_to_string(s_path_rel_file__config).expect("Unable to read the file");
                                        // Parse the JSON data as serde_json::Value
                                        let v2: Value = serde_json::from_str(&s_config_json).unwrap();
                                    
                                        // Access the s_path property directly

                                        if let (Some(s_path_rel_images)) = (
                                            v2["s_path_rel_images"].as_str()
                                        ) {

                                            // Decode the Base64 image data
                                            let o_data_url = DataUrl::parse(s_data_url).expect(&format!("could not parse dataurl {:?}", s_data_url));
                                            let s_mime_type = o_data_url.get_media_type();
                                            let o_path = PathBuf::from(format!("{}/{}/{}.{}", s_path_rel_folder__webroot,s_path_rel_images, s_name_file,f_s_extension_from_s_mime_type(s_mime_type)));
                                            f_ensure_directory(&o_path.parent().unwrap());
                                            let mut o_file = std::fs::File::create(&o_path).unwrap();
                                            o_file.write_all(&o_data_url.get_data()).unwrap();
                                            fs::set_permissions(&o_path, fs::Permissions::from_mode(0o777)).unwrap();
                                            // println!("Image saved successfully.");
                                            // Process the image bytes as needed
                                            o_response.insert("b".to_string(), json!(true));
                                            o_response.insert("s_path_rel".to_string(), json!(
                                                (o_path.iter().skip(2).collect::<PathBuf>())
                                            ));
                                        } else {
                                            println!("Required paths are not specified or not strings in the JSON");
                                        }


                                    }

                                }
                            }
                            if(s_name_function == "f_add_image_to_focus_stack"){


                                if let Some(s_data_url) = v_json_parsed.get("s_data_url").and_then(|v| v.as_str()) {

                                    if let Some(s_name_file) = v_json_parsed.get("s_name_file").and_then(|v| v.as_str()) {


                                        let s_config_json: String = fs::read_to_string(s_path_rel_file__config).expect("Unable to read the file");
                                        // Parse the JSON data as serde_json::Value
                                        let v2: Value = serde_json::from_str(&s_config_json).unwrap();
                                    
                                        // Access the s_path property directly

                                        if let (Some(s_path_rel_stacking)) = (
                                            v2["s_path_rel_stacking"].as_str()
                                        ) {

                                            // Decode the Base64 image data
                                            let o_data_url = DataUrl::parse(s_data_url).expect(&format!("could not parse dataurl {:?}", s_data_url));
                                            let s_mime_type = o_data_url.get_media_type();
                                            let o_path = PathBuf::from(format!("{}/{}/{}.{}", s_path_rel_folder__webroot,s_path_rel_stacking, s_name_file,f_s_extension_from_s_mime_type(s_mime_type)));
                                            f_ensure_directory(&o_path.parent().unwrap());
                                            println!("{:?}", o_path);
                                            let mut o_file = std::fs::File::create(&o_path).unwrap();
                                            o_file.write_all(&o_data_url.get_data()).unwrap();
                                            fs::set_permissions(&o_path, fs::Permissions::from_mode(0o777)).unwrap();

                                            // println!("Image saved successfully.");
                                            // Process the image bytes as needed
                                            o_response.insert("b".to_string(), json!(true));
                                            o_response.insert("s_path_rel".to_string(), json!(
                                                (o_path.iter().skip(2).collect::<PathBuf>())
                                            ));
                                        } else {
                                            println!("Required paths are not specified or not strings in the JSON");
                                        }


                                    }

                                }
                            }
                            if(s_name_function == "f_create_focus_stack"){
                                let s_config_json: String = fs::read_to_string(s_path_rel_file__config).expect("Unable to read the file");
                                // Parse the JSON data as serde_json::Value
                                let v2: Value = serde_json::from_str(&s_config_json).unwrap();
                            
                                // Access the s_path property directly

                                if let (Some(s_path_rel_stacking)) = (
                                    v2["s_path_rel_stacking"].as_str()
                                ) {

                                    println!("asdf");
                                    let o_path = PathBuf::from(format!("{}/{}", s_path_rel_folder__webroot,s_path_rel_stacking));
                                    let o_path_result_image = PathBuf::from(format!("./{}/{}/{}.{}", 
                                    s_path_rel_folder__webroot,
                                    v2["s_path_rel_images"].as_str().unwrap(),
                                    v_json_parsed["s_name_file"].as_str().unwrap(), 
                                    "jpg"
                                    ));
                                    let s_path_images = o_path.as_os_str();
                                    let mut o_command = Command::new(s_path_rel_file__focus_stack_binary);
                                    // o_command.arg("[options]"); // Replace "[options]" with any actual options you need
                                    f_ensure_directory(&o_path_result_image.parent().unwrap());

                                    o_command.arg(format!("--output={}", 
                                    &o_path_result_image.display()
                                    ));
                                    println!("--output={}", 
                                    &o_path_result_image.display()
                                    );
                                
                                    // Read filenames from the directory and add them as arguments
                                    let a_o_entry = fs::read_dir(s_path_images).unwrap();
                                    for o_entry in a_o_entry {
                                        let o_entry = o_entry.unwrap();
                                        let path = o_entry.path();
                                        if path.is_file() 
                                        // && path.extension().map_or(false, |ext| ext == "jpg")
                                        {
                                            o_command.arg(path);
                                        }
                                    }
                                
                                    // Execute the command
                                        // Log the command with its arguments
                                    println!("Executing command: {:?}", o_command);
                                    let o_output = o_command.output().unwrap();
                                    println!("Output: {:?}", String::from_utf8_lossy(&o_output.stdout));
                                    println!("Error: {:?}", String::from_utf8_lossy(&o_output.stderr));



                                    // after done clear tmp dir 
                                    f_create_or_clean_directory(&o_path);

                                } else {
                                    println!("Required paths are not specified or not strings in the JSON");
                                }


                            }

                            if(s_name_function == "f_add_iamge_to_image_stitch"){
                                
                                let s_config_json: String = fs::read_to_string(s_path_rel_file__config).expect("Unable to read the file");
                                let v_config: Value = serde_json::from_str(&s_config_json).unwrap();

                                let o_data_url = DataUrl::parse(v_json_parsed["s_data_url"].as_str().unwrap()).unwrap();
                                let s_mime_type = o_data_url.get_media_type();
                                let o_path = PathBuf::from(format!(
                                    "{}/{}/{}.{}",
                                    s_path_rel_folder__webroot,
                                    v_config["s_path_rel_stitching"].as_str().unwrap(),
                                    v_json_parsed["s_name_file"].as_str().unwrap(),
                                    f_s_extension_from_s_mime_type(s_mime_type))
                                );
                                f_ensure_directory(&o_path.parent().unwrap());
                                fs::set_permissions(&o_path.parent().unwrap(), fs::Permissions::from_mode(0o777)).unwrap();

                                println!("{:?}", o_path);
                                let mut o_file = std::fs::File::create(&o_path).unwrap();
                                o_file.write_all(&o_data_url.get_data()).unwrap();

                                o_response.insert("b".to_string(), json!(true));
                                // if(f_b_directory_contains_more_than_one_image(o_path.parent().unwrap())){
                                    
                                //     // Get the current time
                                //     let o_system_time_now = SystemTime::now();
                                //     // Get the duration since the Unix epoch
                                //     let o = o_system_time_now.duration_since(UNIX_EPOCH)
                                //         .expect("Time went backwards");
                                //     // Convert the duration to milliseconds
                                //     let o_ts = o.as_millis();
                                //     // Format the filename
                                //     let s_name_file = format!("{}_substich_result.png", o_ts);

                                //     let mut o_command = Command::new("python3");
                                //     o_command.arg("image_stitching/image_stitch.py");
                                //     o_command.arg("-i");
                                //     o_command.arg(o_path.parent().unwrap().as_os_str().to_str().unwrap());
                                //     o_command.arg("-o");
                                //     o_command.arg(o_path.parent().unwrap().parent().unwrap().join(s_name_file.clone()));

                                //     println!("Executing command: {:?}", o_command);
                                //     let o_output = o_command.output().unwrap();
                                //     println!("Output: {:?}", String::from_utf8_lossy(&o_output.stdout));
                                //     println!("Error: {:?}", String::from_utf8_lossy(&o_output.stderr));

                                //     let o_path_archive = Path::new(v_config["s_path_rel_archive"].as_str().unwrap());
                                    // if(o_output.status.success()){
                                    //     f_ensure_directory(o_path_archive);
                                    //     for o_entry in fs::read_dir(o_path.parent().unwrap()).unwrap() {
                                    //         let o_entry = o_entry.unwrap();
                                    //         let src_path = o_entry.path();
                                    //         if src_path.is_file() {
                                    //             if let Some(s_name_file2) = src_path.file_name() {
                                    //                 if(s_name_file2.to_str().unwrap() != s_name_file){
                                    //                     let dest_path = o_path_archive.join(s_name_file2);
                                    //                     fs::rename(&src_path, &dest_path).unwrap();
                                    //                 }
                                    //             }
                                    //         }
                                    //     }
                                    //     o_response.insert("s_path_rel".to_string(), json!(
                                    //         (o_path.parent().unwrap().join(s_name_file.clone()).iter().skip(2).collect::<PathBuf>())
                                    //     ));
                                    // }
                                // }
                                // if there are more than 1 images try to create a 'substich'
                                // if substitch successfull remove the images and keep the substitch



                            }
                            if(s_name_function == "f_update_image_stitching_result"){
                                let s_config_json: String = fs::read_to_string(s_path_rel_file__config).expect("Unable to read the file");
                                let v_config: Value = serde_json::from_str(&s_config_json).unwrap();

                                let o_path = PathBuf::from(format!(
                                    "{}/{}/{}.{}",
                                    s_path_rel_folder__webroot,
                                    v_config["s_path_rel_stitching"].as_str().unwrap(),
                                    v_json_parsed["s_name_file"].as_str().unwrap(),
                                    ".tmp")
                                );
                                // Get the current time
                                let o_system_time_now = SystemTime::now();
                                // Get the duration since the Unix epoch
                                let o = o_system_time_now.duration_since(UNIX_EPOCH)
                                    .expect("Time went backwards");
                                // Convert the duration to milliseconds
                                let o_ts = o.as_millis();
                                // Format the filename
                                let s_name_file = format!("{}_substich_result.png", o_ts);

                                let mut o_command = Command::new("python3");
                                o_command.arg("image_stitching/image_stitch.py");
                                o_command.arg("-i");
                                o_command.arg(o_path.parent().unwrap().as_os_str().to_str().unwrap());
                                o_command.arg("-o");
                                o_command.arg(o_path.parent().unwrap().parent().unwrap().join(s_name_file.clone()));

                                println!("Executing command: {:?}", o_command);
                                let o_output = o_command.output().unwrap();
                                println!("Output: {:?}", String::from_utf8_lossy(&o_output.stdout));
                                println!("Error: {:?}", String::from_utf8_lossy(&o_output.stderr));

                                let o_path_archive = Path::new(v_config["s_path_rel_archive"].as_str().unwrap());


                                // println!("Image saved successfully.");
                                // Process the image bytes as needed
                                o_response.insert("b".to_string(), json!(true));
                            }

                            if(s_name_function == "f_s_read_text_file"){
                                o_response.insert("s_json".to_string(), json!(f_s_read_text_file(
                                    v_json_parsed["s_path_rel"].as_str().unwrap(),
                                )));
                            }

                            if(s_name_function == "f_b_write_s_text_file"){

                                f_b_write_s_text_file(
                                    v_json_parsed["s_json"].as_str().unwrap(),
                                    v_json_parsed["s_path_rel"].as_str().unwrap()
                                );
                                o_response.insert("b".to_string(), json!(true));
                            }

                            if(s_name_function == "f_o_command"){
                                if let Some(s_command) = v_json_parsed.get("s_command").and_then(|v: &serde_json::Value| v.as_str()) {

                                    let a_s_arg :Vec<&str> = s_command.split_whitespace().collect();
                                    let a_s_command_allowed = ["lsusb", "touch lol_test"];
                                    if !a_s_command_allowed.contains(&s_command) {
                                        let sresp = format!("command '{}' not allowed, allowed are {:?}", s_command, a_s_command_allowed);
                                        o_tx_sender_clone.send(sresp);
                                    }else{
                                        let a_s_arg2 = if a_s_arg.len() > 1 { &a_s_arg[1..] } else { &[] };
                                        let o_command = Command::new(a_s_arg[0])
                                            .args(a_s_arg2)
                                            .output()
                                            .expect("failed to execute process");
                                        // println!("s_out {:?}", o_command);
                
                                        // Convert output to a string and send it back
                                        let s_stdout = String::from_utf8_lossy(&o_command.stdout);
                                        let s_stderr = String::from_utf8_lossy(&o_command.stderr);
                                        let o_status = o_command.status;
                                        o_response.insert("n_return_code".to_string(), json!(o_status.code()));
                                        o_response.insert("s_stdout".to_string(), json!(s_stdout));
                                        o_response.insert("s_stderr".to_string(), json!(s_stderr));

                                        // println!("message {:?}", sdev);
                                        // i want to be able to send data here... 
                                    }
                                }
                            }

                            
                            o_response.insert("s_uuid".to_string(), json!(s_uuid));
                            o_tx_sender_clone.send(
                                serde_json::to_string(&Value::Object(o_response)).unwrap()
                            );
                        }
                        
                        println!("received s_name_function {}", s_name_function);
                        // o_tx_sender_clone.send(String::from("message received, heres a new one sent!"));
                        
                        if(s_name_function == "f_control_stepper_motor"){

                            if let Some(s_axis) = v_json_parsed.get("s_axis").and_then(|v: &serde_json::Value| v.as_str()) {

                                let n_rpm_nor = &v_json_parsed["n_rpm_nor"];
                                if n_rpm_nor.is_number(){
                                    println!("asdf: {}", n_rpm_nor.as_f64().unwrap());
                                }
                                
                                if(s_axis == "x"){
                                    if let Some(ref o) = v_o_sender_tx_stepper_28BYJ_48_x{
                                        o.send(
                                            (v_json_parsed.clone()).to_string()
                                        ).unwrap();
                                    }
                                }
                                if(s_axis == "y"){
                                    if let Some(ref o) = v_o_sender_tx_stepper_28BYJ_48_y{
                                        o.send(
                                            (v_json_parsed.clone()).to_string()
                                        ).unwrap();
                                    }
                                }
                                if(s_axis == "z"){

                                    if let Some(ref o) = v_o_sender_tx_stepper_28BYJ_48_z{
                                        o.send(
                                            (v_json_parsed.clone()).to_string()
                                        ).unwrap();
                                    }
                                }


                            }
                            // else{
                            //     o_response.insert("s_error".to_string(), json!("the property 's_axis' must be included"));
                            // }


                        }



                        if(s_name_function == "f_switch_usb_device"){

                            let n_id_vendor = v_json_parsed.get("n_id_vendor").unwrap().as_i64().expect("value has to be number");
                            let n_id_product = v_json_parsed.get("n_id_product").unwrap().as_i64().expect("value has to be number");

                            o_tx_control_usb.send(ControlCommand::Stop).unwrap();
                            o_tx_control_usb.send(ControlCommand::SwitchDevice(
                                n_id_vendor as u16, 
                                n_id_product as u16, 
                            )).unwrap();
                            o_tx_control_usb.send(ControlCommand::Start).unwrap();
                        }


                        
                    }
                }
            }
        }

    }

    // Make sure the forwarding task is properly cleaned up
    let _ = forward_task.await;
}

#[tokio::main]
async fn main() {

    

    f_install_denojs();
    //raspi pinout pin layout 
    // |----------------------|----------------------|
    // |_   3v3 power         |_   5v power          |
    // |_   GPIO 2 (SDA)      |_   5v power          |
    // |_   GPIO 3 (SCL)      |_   Ground            |
    // |_   GPIO 4 (GPCLK0)   |_   GPIO 14 (TXD)     |
    // |_   Ground            |_   GPIO 15 (RXD)     |
    // |_   GPIO 17           |_   GPIO 18 (PCM_CLK) |
    // |_   GPIO 27           |_   Ground            |
    // |_   GPIO 22           |_   GPIO 23           |
    // |_   3v3 power         |_   GPIO 24           |
    // |_   GPIO 10 (MOSI)    |_   Ground            |
    // |_   GPIO 9 (MISO)     |_   GPIO 25           |
    // |_   GPIO 11 (SCLK)    |_   GPIO 8 (CEO)      |
    // |_   Ground            |_   GPIO 7 (CE1)      |
    // |_   GPIO 0 (ID_SD)    |_   GPIO 1 (ID_SD)    |
    // |_   GPIO 5            |_   Ground            |
    // |_   GPIO 6            |_   GPIO 12 (PWM0)    |
    // |_   GPIO 13 (PWM1)    |_   Ground            |
    // |_   GPIO 19 (PCM_FS)  |_   GPIO 16           |
    // |_   GPIO 26           |_   GPIO 20 (PCM_DIN) |
    // |_   Ground            |_   GPIO 21 (PCM_DOUT)|
    // |----------------------|----------------------|

    let o_path_webserver_root = Path::new(s_path_rel_folder__webroot);
    let b_gpio_available = f_b_gpio_available();
    
    let mut v_o_sender_tx_stepper_28BYJ_48_x = None;
    let mut v_o_sender_tx_stepper_28BYJ_48_y = None;
    let mut v_o_sender_tx_stepper_28BYJ_48_z = None;

    if(b_gpio_available){
        let a_n_pin_x = [2,3,4,17];
        let a_n_pin_y = [27,22,10,9];
        let a_n_pin_z = [11,0,5,6];
        v_o_sender_tx_stepper_28BYJ_48_x = Some(f_o_sender_tx_spawn_thread_with_event_listener_for_stepper(a_n_pin_x));
        v_o_sender_tx_stepper_28BYJ_48_y = Some(f_o_sender_tx_spawn_thread_with_event_listener_for_stepper(a_n_pin_y));
        v_o_sender_tx_stepper_28BYJ_48_z = Some(f_o_sender_tx_spawn_thread_with_event_listener_for_stepper(a_n_pin_z));
        println!("you are using a raspberry pi, make sure the stepper motors are wired like this");
        println!("x-axis stepper gpio pins: {:?}", a_n_pin_x);
        println!("y-axis stepper gpio pins: {:?}", a_n_pin_y);
        println!("z-axis stepper gpio pins: {:?}", a_n_pin_z);
    }else{
        println!("with some 3d printed hardware and byj28 stepper motors you can run this software to control the microscope with a usb controller")
    }

    let mut v_o_sender_tx_stepper_28BYJ_48_x__clone_for_main = v_o_sender_tx_stepper_28BYJ_48_x.clone();
    let mut v_o_sender_tx_stepper_28BYJ_48_y__clone_for_main = v_o_sender_tx_stepper_28BYJ_48_y.clone();
    let mut v_o_sender_tx_stepper_28BYJ_48_z__clone_for_main = v_o_sender_tx_stepper_28BYJ_48_z.clone();


    let (o_tx_control_usb1, o_rx_control_usb1) = mpsc::channel();
    let (o_tx_control_usb2, o_rx_control_usb2) = mpsc::channel();

    thread::spawn(move || {
        f_usb_read_thread(o_tx_control_usb2, o_rx_control_usb1);
    });

    let n_messages_at_a_time_in_the_channel_buffer_max = 10;
    let (o_tx_sender, _) = broadcast::channel::<String>(n_messages_at_a_time_in_the_channel_buffer_max);
    let o_tx_sender_clone = o_tx_sender.clone();

    let routes = warp::path("ws")
        .and(warp::ws())
        .map(move |o_ws: warp::ws::Ws| {
            let o_rx_receiver: broadcast::Receiver<String> = o_tx_sender.subscribe();
            let o_tx_sender_clone = o_tx_sender.clone();
            let o_tx_control_usb1_clone = o_tx_control_usb1.clone();
            let mut v_o_sender_tx_stepper_28BYJ_48_x__clone_for_websocket = v_o_sender_tx_stepper_28BYJ_48_x.clone();
            let mut v_o_sender_tx_stepper_28BYJ_48_y__clone_for_websocket = v_o_sender_tx_stepper_28BYJ_48_y.clone();
            let mut v_o_sender_tx_stepper_28BYJ_48_z__clone_for_websocket = v_o_sender_tx_stepper_28BYJ_48_z.clone();

            o_ws.on_upgrade(move |o_websocket| f_websocket_thread(
                o_websocket,
                o_rx_receiver,
                o_tx_sender_clone,
                o_tx_control_usb1_clone,
                v_o_sender_tx_stepper_28BYJ_48_x__clone_for_websocket,
                v_o_sender_tx_stepper_28BYJ_48_y__clone_for_websocket,
                v_o_sender_tx_stepper_28BYJ_48_z__clone_for_websocket
            ))
        })
        .or(warp::fs::dir(o_path_webserver_root));
    //use o_sender_tx_stepper_28BYJ_48_y here later
    let mut a_n_ip = [127,0,0,1];
    let s_hostname = gethostname();
    if(s_hostname == "raspi-desktop"){
        a_n_ip = [192,168,1,105];
    }
    tokio::spawn(
        warp::serve(routes)
        .tls()
        // RSA
        .cert_path("cert.pem")
        .key_path("key.pem")
        .run((a_n_ip, 3030))
    );
    println!("webserver running at http://127.0.0.1:3030/");
    println!("websocket running at ws://127.0.0.1:3030/ws");



    while let Ok(o) = o_rx_control_usb2.recv() {
        // println!("Received data: {:?}", data.a_n_u8_usb_read_result);
    
        if let Some(o_input_device) = o.v_o_input_device{

            o_tx_sender_clone.send(serde_json::to_string(
                &json!({
                    "o_input_device": o_input_device  // Embed the struct within the outer property
                })
            ).expect("failed to convert to json"));
            // println!("update");
            let o_left_stick_x_axis = f_o_input_sensor_from_s_name(&o_input_device, "left_stick_x_axis").unwrap();
            let o_left_stick_y_axis = f_o_input_sensor_from_s_name(&o_input_device, "left_stick_y_axis").unwrap();
            let o_right_stick_x_axis = f_o_input_sensor_from_s_name(&o_input_device, "right_stick_x_axis").unwrap();
            let o_right_stick_y_axis = f_o_input_sensor_from_s_name(&o_input_device, "right_stick_y_axis").unwrap();
    
            let mut n_l_x = (o_left_stick_y_axis.n_nor-0.5)*2.;
            let mut n_l_y = (o_left_stick_x_axis.n_nor-0.5)*2.;
            let mut n_r_x = (o_right_stick_x_axis.n_nor-0.5)*2.;
            let mut n_r_y = (o_right_stick_y_axis.n_nor-0.5)*2.;
    
            // if the gamepad is connected and autmatically turned of 
            // to save power, the x axis will still be interpreted as 0
            // and then 0 will be sent
            // therefore we have to check if the value differs from the last one... 

            // for o in o_input_device.a_o_input_sensor.iter(){
            //     println!("{}:{}", o.s_name, o.n_nor);
            // }
            // n_l_x = if(n_l_x.abs() > 0.05){n_l_x*0.5} else{0.0};
            // n_l_y = if(n_l_y.abs() > 0.05){n_l_y*0.5} else{0.0};
            // n_r_x = if(n_r_x.abs() > 0.05){n_r_x*0.5} else{0.0};
            // n_r_y = if(n_r_y.abs() > 0.05){n_r_y*0.5} else{0.0};
            // println!("n_r_x,n_r_y,n_l_x,n_l_y {},{},{},{}", n_r_x,n_r_y,n_l_x,n_l_y);


            if let Some(ref o) = v_o_sender_tx_stepper_28BYJ_48_x__clone_for_main{
                o.send(
                    json!({ 
                        "n_rpm_nor": n_r_x.abs(),
                        "b_direction": if(n_r_x>0.0){true}else{false}
                    }).to_string()
                ).unwrap();
            }
            if let Some(ref o) = v_o_sender_tx_stepper_28BYJ_48_y__clone_for_main{
                o.send(
                    json!({ 
                        "n_rpm_nor": n_r_y.abs(),
                        "b_direction": if(n_r_y>0.0){true}else{false}
                    }).to_string()
                ).unwrap();
            }
            if let Some(ref o) = v_o_sender_tx_stepper_28BYJ_48_z__clone_for_main{
                o.send(
                    json!({ 
                        "n_rpm_nor": n_l_y.abs(),
                        "b_direction": if(n_l_y>0.0){true}else{false}
                    }).to_string()
                ).unwrap();
            }
        }

        // tx_clone.send(data.a_n_u8_usb_read_result.unwrap());
        // tx_clone.send(vec![12,2,2,3,]).unwrap();

    }

    
}

