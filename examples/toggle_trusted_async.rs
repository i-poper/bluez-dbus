use bluez_dbus::nonblock::{Device, Session};
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let s = Session::new().unwrap();
    let mut args = env::args();
    let dev_path = args.nth(1).unwrap();
    let dev = Device::new(&s, &dev_path);
    let trusted = dev.is_trusted().await?;
    println!("now: {}", trusted);
    if trusted {
        dev.set_trusted(false).await?;
    } else {
        dev.set_trusted(true).await?;
    }
    let trusted = dev.is_trusted().await?;
    println!("change: {}", trusted);
    Ok(())
}
