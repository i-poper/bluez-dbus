use crate::nonblock::Session;
use crate::*;
use dbus::arg::{Append, Arg, Get};
use dbus::strings::Path;

static DEVICE_INTERFACE: &str = "org.bluez.Device1";

#[derive(Debug)]
pub struct Device {
    session: Session,
    path: String,
}

impl Device {
    /// デバイス作成
    pub fn new(session: &Session, path: &str) -> Self {
        Device {
            session: session.clone(),
            path: path.to_string(),
        }
    }

    /// デバイスのオブジェクトパスを取得
    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    /// デバイスに属するgattサービスの一覧を取得
    pub async fn get_gatt_services(&self) -> Result<Option<Vec<String>>, BoxError> {
        self.session.get_children(&self.path, "Device").await
    }

    pub async fn connect(&self) -> Result<(), BoxError> {
        Ok(self
            .session
            .method_call(&self.path, DEVICE_INTERFACE, "Connect", ())
            .await?)
    }

    pub async fn disconnect(&self) -> Result<(), BoxError> {
        Ok(self
            .session
            .method_call(&self.path, DEVICE_INTERFACE, "Disconnect", ())
            .await?)
    }

    pub async fn connect_profile(&self, value: &str) -> Result<(), BoxError> {
        Ok(self
            .session
            .method_call(&self.path, DEVICE_INTERFACE, "ConnectProfile", (value,))
            .await?)
    }

    pub async fn disconnect_profile(&self, value: &str) -> Result<(), BoxError> {
        Ok(self
            .session
            .method_call(&self.path, DEVICE_INTERFACE, "DisconnectProfile", (value,))
            .await?)
    }

    pub async fn pair(&self) -> Result<(), BoxError> {
        Ok(self
            .session
            .method_call(&self.path, DEVICE_INTERFACE, "Pair", ())
            .await?)
    }

    pub async fn cancel_pairing(&self) -> Result<(), BoxError> {
        Ok(self
            .session
            .method_call(&self.path, DEVICE_INTERFACE, "CancelPairing", ())
            .await?)
    }

    async fn get_property<A: for<'z> Get<'z> + 'static>(
        &self,
        property: &str,
    ) -> Result<A, BoxError> {
        Ok(self
            .session
            .get_property(&self.path, DEVICE_INTERFACE, property)
            .await?)
    }
    async fn set_property<T: Append + Arg>(&self, prop: &str, value: T) -> Result<(), BoxError> {
        Ok(self
            .session
            .set_property(&self.path, DEVICE_INTERFACE, prop, value)
            .await?)
    }

    //--------------------------------------------------------------------------------
    // プロパティ
    // get
    async_get_property!(get_address, String, "Address");
    async_get_property!(get_name, String, "Name");
    async_get_property!(get_icon, String, "Icon");
    async_get_property!(get_class, u32, "Class");
    async_get_property!(get_appearance, u16, "Appearance");
    async_get_property!(get_uuids, Vec<String>, "UUIDs");
    async_get_property!(is_paired, bool, "Paired");
    async_get_property!(is_connected, bool, "Connected");
    async_get_property!(is_trusted, bool, "Trusted");
    async_get_property!(is_blocked, bool, "Blocked");
    async_get_property!(get_alias, String, "Alias");
    async_get_property!(get_adapter, Path<'_>, "Adapter");
    async_get_property!(is_legacy_pairing, bool, "LegacyPairing");
    async_get_property!(get_modalias, String, "Modalias");
    async_get_property!(get_rssi, i16, "RSSI");
    async_get_property!(get_tx_power, i16, "TxPower");
    // set
    async_set_property!(set_trusted, bool, "Trusted");
    async_set_property!(set_blocked, bool, "Blocked");
    async_set_property!(set_alias, String, "Alias");
}
