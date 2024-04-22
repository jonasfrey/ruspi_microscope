use blurz::{BluetoothAdapter, BluetoothSession, BluetoothDevice};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = BluetoothSession::create_session(None)?;
    let adapter = BluetoothAdapter::init(&session)?;
    adapter.set_powered(true)?;
    adapter.start_discovery()?;
    std::thread::sleep(std::time::Duration::from_secs(4));
    adapter.stop_discovery()?;

    let devices = adapter.get_device_list()?;
    for device_path in devices {
        let device = BluetoothDevice::new(&session, device_path);
        println!("Found device: {} {}", device.get_name()?, device.get_address()?);
        // If this is the DualSense device, you can now attempt to connect to it and interact further
    }

    Ok(())
}