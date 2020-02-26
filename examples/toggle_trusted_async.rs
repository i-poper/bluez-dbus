use bluez_dbus::nonblock::{Device, Session};
use std::env;
use std::error::Error;
use tokio::runtime::Builder;

#[cfg(not(feature = "local"))]
pub fn main() -> Result<(), Box<dyn Error>> {
    let mut rt = Builder::new().enable_all().threaded_scheduler().build()?;
    rt.block_on(async { process().await })?;
    Ok(())
}

#[cfg(feature = "local")]
pub fn main() -> Result<(), Box<dyn Error>> {
    let local = tokio::task::LocalSet::new();
    let mut rt = Builder::new().enable_all().basic_scheduler().build()?;
    local.block_on(&mut rt, async { process().await })?;
    Ok(())
}

async fn process() -> Result<(), Box<dyn Error>> {
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
