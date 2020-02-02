use crate::blocking::Session;
use crate::*;
use dbus::arg::{Append, Arg, Get};

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

    pub fn connect(&self) -> Result<(), BoxError> {
        Ok(self.session.method_call(&self.path, DEVICE_INTERFACE, "Connect", ())?)
    }

    pub fn disconnect(&self) -> Result<(), BoxError> {
        Ok(self.session.method_call(&self.path, DEVICE_INTERFACE, "Disconnect", ())?)
    }

    pub fn connect_profile(&self, value: &str) -> Result<(), BoxError> {
        Ok(self.session.method_call(&self.path, DEVICE_INTERFACE, "ConnectProfile", (value,))?)
    }

    pub fn disconnect_profile(&self, value: &str) -> Result<(), BoxError> {
        Ok(self.session.method_call(&self.path, DEVICE_INTERFACE, "DisconnectProfile", (value,))?)
    }

    pub fn pair(&self) -> Result<(), BoxError> {
        Ok(self.session.method_call(&self.path, DEVICE_INTERFACE, "Pair", ())?)
    }

    pub fn cancel_pairing(&self) -> Result<(), BoxError> {
        Ok(self.session.method_call(&self.path, DEVICE_INTERFACE, "CancelPairing", ())?)
    }

    fn get_property<A: for<'z> Get<'z>>(&self, property: &str) -> Result<A, BoxError> {
        Ok(self
            .session
            .get_property(&self.path, DEVICE_INTERFACE, property)?)
    }
    fn set_property<T: Append + Arg>(&self, prop: &str, value: T) -> Result<(), BoxError> {
        Ok(self
            .session
            .set_property(&self.path, DEVICE_INTERFACE, prop, value)?)
    }

    //--------------------------------------------------------------------------------
    // プロパティ
    // get
    get_property!(get_address, String, "Address");
    get_property!(get_name, String, "Name");
    get_property!(get_icon, String, "Icon");
    get_property!(get_class, u32, "Class");
    get_property!(get_appearance, u16, "Appearance");
    get_property!(get_uuids, Vec<String>, "UUIDs");
    get_property!(is_paired, bool, "Paired");
    get_property!(is_connected, bool, "Connected");
    get_property!(is_trusted, bool, "Trusted");
    get_property!(is_blocked, bool, "Blocked");
    get_property!(get_alias, String, "Alias");
    get_property!(get_adapter, String, "Adapter");
    get_property!(is_legacy_pairing, bool, "LegacyPairing");
    get_property!(get_modalias, String, "Modalias");
    get_property!(get_rssi, i16, "RSSI");
    get_property!(get_tx_power, i16, "TxPower");
    // set
    set_property!(set_trusted, bool, "Trusted");
    set_property!(set_blocked, bool, "Blocked");
    set_property!(set_alias, String, "Alias");
}
