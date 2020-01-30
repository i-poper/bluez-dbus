use crate::blocking::Session;
use crate::*;
use dbus::arg::{Append, Arg, Get, Variant};

static DEVICE_INTERFACE: &str = "org.bluez.Device1";

pub struct Device<'a> {
    session: &'a Session,
    path: String,
}

impl<'a> Device<'a> {
    /// デバイス作成
    pub fn new(session: &'a Session, path: &str) -> Self {
        Device {
            session,
            path: path.to_string(),
        }
    }

    /// デバイスのオブジェクトパスを取得
    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    /// デバイスに属するgattサービスの一覧を取得
    pub fn get_gatt_services(&self) -> Result<Option<Vec<String>>, BoxError> {
        self.session.get_children(&self.path, "Device")
    }

    fn get_property<T: for<'z> Get<'z>>(&self, prop: &str) -> Result<T, BoxError> {
        let (value,): (Variant<T>,) =
            self.session
                .get_property(DEVICE_INTERFACE, &self.path, prop)?;
        Ok(value.0)
    }

    fn set_property<T: Append + Arg>(&self, prop: &str, value: T) -> Result<(), BoxError> {
        let value = Variant(value);
        self.session
            .set_property(DEVICE_INTERFACE, &self.path, prop, value)?;
        Ok(())
    }

    //--------------------------------------------------------------------------------
    // プロパティ
    pub fn get_address(&self) -> Result<String, BoxError> {
        self.get_property("Address")
    }

    pub fn get_name(&self) -> Result<String, BoxError> {
        self.get_property("Name")
    }

    pub fn get_icon(&self) -> Result<String, BoxError> {
        self.get_property("Icon")
    }

    pub fn is_paired(&self) -> Result<bool, BoxError> {
        self.get_property("Paired")
    }

    pub fn is_connected(&self) -> Result<bool, BoxError> {
        self.get_property("Connected")
    }

    pub fn is_trusted(&self) -> Result<bool, BoxError> {
        self.get_property("Trusted")
    }

    pub fn set_trusted(&self, value: bool) -> Result<(), BoxError> {
        self.set_property("Trusted", value)
    }
}
