use bluez_dbus::blocking::{Adapter, Device, Session};
use std::error::Error;
use std::thread;
use std::time::Duration;

pub fn main() -> Result<(), Box<dyn Error>> {
    let s = Session::new().unwrap();
    let adapters = Adapter::list(&s)?;
    let adapters = adapters.unwrap();
    adapters.iter().for_each(|adapter| {
        println!("{}", adapter);
    });

    let adapter_path = &adapters[0];
    let adapter = Adapter::create(&s, adapter_path).unwrap().unwrap();
    adapter.start_discovery().unwrap();
    thread::sleep(Duration::from_millis(2000));

    if let Some(a) = Adapter::create(&s, adapter_path)? {
        if let Some(devices) = a.device_list()? {
            let devices: Vec<Device> = devices
                .iter()
                .map(|device| Device::new(&s, device))
                .collect();
            devices.iter().for_each(|dev| {
                let _ = print_dev(&dev);
            });
        }
    } else {
        println!("not found: {}", adapter_path);
    }
    adapter.stop_discovery().unwrap();
    Ok(())
}

fn print_dev(dev: &Device) -> Result<(), Box<dyn Error>> {
    println!("【{}】", dev.get_name()?);
    if let Ok(address) = dev.get_address() {
        println!("  address: {}", address);
    }
    println!("  trusted: {}", dev.is_trusted()?);
    println!("  paired : {}", dev.is_paired()?);
    if let Ok(icon) = dev.get_icon() {
        println!("  icon   : {}", icon);
    }
    if let Ok(Some(gatts)) = dev.get_gatt_services() {
        gatts.iter().for_each(|gatt| {
            println!("    {}", gatt);
        });
    }
    Ok(())
}
