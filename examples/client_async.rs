use bluez_dbus::nonblock::{Adapter, Device, GattService, Session};
use std::error::Error;
use std::thread;
use std::time::Duration;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let s = Session::new().unwrap();
    let adapters = s.get_adapters().await?;
    let adapters = adapters.unwrap();
    adapters.iter().for_each(|adapter| {
        println!("{}", adapter);
    });

    let adapter_path = &adapters[0];
    let adapter = Adapter::create(&s, adapter_path).await.unwrap().unwrap();
    adapter.start_discovery().await.unwrap();
    thread::sleep(Duration::from_millis(2000));

    if let Some(a) = Adapter::create(&s, adapter_path).await? {
        if let Some(devices) = a.get_devices().await? {
            let devices: Vec<Device> = devices
                .iter()
                .map(|device| Device::new(&s, device))
                .collect();
            for dev in devices.iter() {
                print_dev(&s, &dev).await?;
            }
        }
    } else {
        println!("not found: {}", adapter_path);
    }
    adapter.stop_discovery().await.unwrap();
    Ok(())
}

async fn print_dev(session: &Session, dev: &Device) -> Result<(), Box<dyn Error>> {
    println!("【{}】", match dev.get_name().await {
        Ok(name) => name,
        _ => "no_name".to_string(),
    });
    if let Ok(address) = dev.get_address().await {
        println!("  address: {}", address);
    }
    println!("     path: {}", dev.get_path());
    println!("  trusted: {}", dev.is_trusted().await?);
    println!("   paired: {}", dev.is_paired().await?);
    println!("  Adapter: {}", dev.get_adapter().await?);
    println!("    UUIDs: {:?}", dev.get_uuids().await?);
    if let Ok(icon) = dev.get_icon().await {
        println!("     icon: {}", icon);
    }
    if let Ok(Some(gatts)) = dev.get_gatt_services().await {
        for gatt in gatts.iter() {
            print_gatt(session, gatt).await?;
        }
    }
    Ok(())
}

async fn print_gatt(session: &Session, gatt: &str) -> Result<(), Box<dyn Error>> {
    println!("Gatt Service: {}", gatt);
    let gatt_service = GattService::new(session, gatt);
    println!(" device:{}", gatt_service.get_device().await?);
    if let Ok(Some(chars)) = gatt_service.get_characteristics().await {
        chars.iter().for_each(|charc| {
            println!("    {}", charc);
        });
    }
    Ok(())
}
