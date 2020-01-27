use bluez_dbus::blocking::{Session, Adapter};

pub fn main() {
    let s = Session::new().unwrap();
    if let Some(adapters) = Adapter::list(&s).unwrap() {
        adapters.iter().for_each( |adapter| {
            println!("{}", adapter );
        });
    }
    let adapter_path = "/org/bluez/hci0";
    if let Some(a) = Adapter::create(&s,adapter_path ).unwrap() {
        if let Some(devices) = a.device_list().unwrap() {
            devices.iter().for_each( |device| {
                println!("device: {}", device);
            });
        }
    }
    else {
        println!("not found: {}",adapter_path);
    }
}
