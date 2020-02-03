use crate::blocking::Session;
use crate::*;
use dbus::arg::{Append, Arg, Get};
use std::error::Error;

#[derive(Debug)]
pub struct Adapter<'a> {
    session: &'a Session,
    path: String,
}

impl<'a> Adapter<'a> {
    fn new(session: &'a Session, path: &str) -> Self {
        Adapter {
            session,
            path: path.to_string(),
        }
    }

    /// bluetoothアダプターの作成
    ///
    /// 指定されたパスの存在を確認してアダプターを作成する。
    /// 存在しない場合は`Ok(None)`を返す。
    pub fn create(
        session: &'a Session,
        path: &str,
    ) -> Result<Option<Self>, Box<dyn Error + 'static>> {
        if let Some(adapters) = session.get_adapters()? {
            if adapters.contains(&path.to_string()) {
                return Ok(Some(Adapter::new(session, path)));
            }
        }
        Ok(None)
    }

    /// デバイスリスト取得
    ///
    /// アダプターに登録されているデバイスのパスのリストを取得する
    pub fn get_devices(&self) -> Result<Option<Vec<String>>, BoxError> {
        self.session.get_children(&self.path, "Adapter")
    }

    /// デバイスの検索を開始する
    pub fn start_discovery(&self) -> Result<(), BoxError> {
        self.sub_discovery("StartDiscovery")
    }

    /// デバイスの検索を停止する
    pub fn stop_discovery(&self) -> Result<(), BoxError> {
        self.sub_discovery("StopDiscovery")
    }

    pub fn remove_device(&self, device: &str) -> Result<(), BoxError> {
        self.session
            .method_call(&self.path, ADAPTER_INTERFACE, "RemoveDevice", (device,))?;
        Ok(())
    }

    // TODO: SetDiscoveryFilter

    fn sub_discovery(&self, method: &str) -> Result<(), BoxError> {
        self.session
            .method_call(&self.path, ADAPTER_INTERFACE, method, ())?;
        Ok(())
    }

    fn get_property<A: for<'z> Get<'z>>(&self, property: &str) -> Result<A, BoxError> {
        Ok(self
            .session
            .get_property(&self.path, ADAPTER_INTERFACE, property)?)
    }
    fn set_property<T: Append + Arg>(&self, prop: &str, value: T) -> Result<(), BoxError> {
        Ok(self
            .session
            .set_property(&self.path, ADAPTER_INTERFACE, prop, value)?)
    }

    //--------------------------------------------------------------------------------
    // プロパティ
    // get
    get_property!(get_address, String, "Address");
    get_property!(get_name, String, "Name");
    get_property!(get_alias, String, "Alias");
    get_property!(get_class, u32, "Class");
    get_property!(is_powered, bool, "Powered");
    get_property!(is_discoverable, bool, "Discoverable");
    get_property!(is_pairable, bool, "Pairable");
    get_property!(get_pairable_timeout, u32, "PairableTimeout");
    get_property!(get_discoverable_timeout, u32, "DiscoverableTimeout");
    get_property!(is_discovering, bool, "Discovering");
    get_property!(get_uuids, Vec<String>, "UUIDs");
    get_property!(get_modalias, String, "Modalias");
    // set
    set_property!(set_alias, String, "Alias");
    set_property!(set_powered, bool, "Powered");
    set_property!(set_discoverable, bool, "Discoverable");
    set_property!(set_pairable, bool, "Pairable");
    set_property!(set_pairable_timeout, u32, "PairableTimeout");
    set_property!(set_discoverable_timeout, u32, "DiscoverableTimeout");
}
