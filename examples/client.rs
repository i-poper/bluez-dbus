use bluez_dbus::blocking::{Adapter, Device, GattService, Session};
use std::error::Error;
use std::thread;
use std::time::Duration;

pub fn main() -> Result<(), Box<dyn Error>> {
    let s = Session::new().unwrap();
    let adapters = s.get_adapters()?;
    let adapters = adapters.unwrap();
    adapters.iter().for_each(|adapter| {
        println!("{}", adapter);
    });

    let adapter_path = &adapters[0];
    let adapter = Adapter::create(&s, adapter_path).unwrap().unwrap();
    adapter.start_discovery().unwrap();
    thread::sleep(Duration::from_millis(2000));

    if let Some(a) = Adapter::create(&s, adapter_path)? {
        if let Some(devices) = a.get_devices()? {
            let devices: Vec<Device> = devices
                .iter()
                .map(|device| Device::new(&s, device))
                .collect();
            for dev in devices.iter() {
                print_dev(&s, &dev)?;
            }
        }
    } else {
        println!("not found: {}", adapter_path);
    }
    adapter.stop_discovery().unwrap();
    Ok(())
}

fn print_dev(session: &Session, dev: &Device) -> Result<(), Box<dyn Error>> {
    println!("【{}】", match dev.get_name() {
        Ok(name) => name,
        _ => "no_name".to_string(),
    });
    if let Ok(address) = dev.get_address() {
        println!("  address: {}", address);
    }
    println!("     path: {}", dev.get_path());
    println!("  trusted: {}", dev.is_trusted()?);
    println!("   paired: {}", dev.is_paired()?);
    println!("  Adapter: {}", dev.get_adapter()?);
    println!("    UUIDs: {:?}", dev.get_uuids()?);
    if let Ok(icon) = dev.get_icon() {
        println!("     icon: {}", icon);
    }
    if let Ok(Some(gatts)) = dev.get_gatt_services() {
        for gatt in gatts.iter() {
            print_gatt(session, gatt)?;
        }
    }
    Ok(())
}

fn print_gatt(session: &Session, gatt: &str) -> Result<(), Box<dyn Error>> {
    println!("Gatt Service: {}", gatt);
    let gatt_service = GattService::new(session, gatt);
    println!(" device:{}", gatt_service.get_device()?);
    if let Ok(Some(chars)) = gatt_service.get_characteristics() {
        chars.iter().for_each(|charc| {
            println!("    {}", charc);
        });
    }
    Ok(())
}
