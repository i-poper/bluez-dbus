use crate::blocking::Session;
use crate::*;
use dbus::arg::Variant;

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

    fn get_property_string(&self, prop: &str) -> Result<String, BoxError> {
        let value = self
            .session
            .get_property("org.bluez.Device1", &self.path, prop)?;
        Ok(value.as_str().unwrap().to_string())
    }

    fn get_property_bool(&self, prop: &str) -> Result<bool, BoxError> {
        let value = self
            .session
            .get_property("org.bluez.Device1", &self.path, prop)?;
        let b = value.as_i64().unwrap();
        if b == 1 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn set_property_bool(&self, prop: &str, value: bool) -> Result<(), BoxError> {
        let value = Variant(value);
        self.session
            .set_property("org.bluez.Device1", &self.path, prop, value)?;
        Ok(())
    }

    //--------------------------------------------------------------------------------
    // プロパティ
    pub fn get_address(&self) -> Result<String, BoxError> {
        self.get_property_string("Address")
    }
    pub fn get_name(&self) -> Result<String, BoxError> {
        self.get_property_string("Name")
    }

    pub fn get_icon(&self) -> Result<String, BoxError> {
        self.get_property_string("Icon")
    }

    pub fn is_paired(&self) -> Result<bool, BoxError> {
        self.get_property_bool("Paired")
    }

    pub fn is_connected(&self) -> Result<bool, BoxError> {
        self.get_property_bool("Connected")
    }

    pub fn is_trusted(&self) -> Result<bool, BoxError> {
        self.get_property_bool("Trusted")
    }

    pub fn set_trusted(&self, value: bool) -> Result<(), BoxError> {
        self.set_property_bool("Trusted", value)
    }
}
