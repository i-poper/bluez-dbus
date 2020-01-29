use bluez_dbus::blocking::{Device, Session};
use std::env;
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    let s = Session::new().unwrap();
    let mut args = env::args();
    let dev_path = args.nth(1).unwrap();
    let dev = Device::new(&s, &dev_path);
    let trusted = dev.is_trusted()?;
    println!("now: {}", trusted);
    if trusted {
        dev.set_trusted(false)?;
    } else {
        dev.set_trusted(true)?;
    }
    let trusted = dev.is_trusted()?;
    println!("change: {}", trusted);
    Ok(())
}
