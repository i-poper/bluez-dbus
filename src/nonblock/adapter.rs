use crate::nonblock::Session;
use crate::*;
use dbus::arg::{Append, Arg, Get};
use std::error::Error;

#[derive(Debug)]
pub struct Adapter {
    session: Session,
    path: String,
}

impl Adapter {
    fn new(session: &Session, path: &str) -> Self {
        Adapter {
            session: session.clone(),
            path: path.to_string(),
        }
    }

    /// bluetoothアダプターの作成
    ///
    /// 指定されたパスの存在を確認してアダプターを作成する。
    /// 存在しない場合は`Ok(None)`を返す。
    pub async fn create(
        session: &Session,
        path: &str,
    ) -> Result<Option<Self>, Box<dyn Error + 'static>> {
        if let Some(adapters) = session.get_adapters().await? {
            if adapters.contains(&path.to_string()) {
                return Ok(Some(Adapter::new(session, path)));
            }
        }
        Ok(None)
    }

    /// デバイスリスト取得
    ///
    /// アダプターに登録されているデバイスのパスのリストを取得する
    pub async fn get_devices(&self) -> Result<Option<Vec<String>>, BoxError> {
        self.session.get_children(&self.path, "Adapter").await
    }

    /// デバイスの検索を開始する
    pub async fn start_discovery(&self) -> Result<(), BoxError> {
        self.sub_discovery("StartDiscovery").await
    }

    /// デバイスの検索を停止する
    pub async fn stop_discovery(&self) -> Result<(), BoxError> {
        self.sub_discovery("StopDiscovery").await
    }

    pub async fn remove_device(&self, device: &str) -> Result<(), BoxError> {
        self.session
            .method_call(&self.path, ADAPTER_INTERFACE, "RemoveDevice", (device,))
            .await?;
        Ok(())
    }

    // TODO: SetDiscoveryFilter

    async fn sub_discovery(&self, method: &str) -> Result<(), BoxError> {
        self.session
            .method_call(&self.path, ADAPTER_INTERFACE, method, ())
            .await?;
        Ok(())
    }

    async fn get_property<A: for<'z> Get<'z> + 'static>(
        &self,
        property: &str,
    ) -> Result<A, BoxError> {
        Ok(self
            .session
            .get_property(&self.path, ADAPTER_INTERFACE, property)
            .await?)
    }
    async fn set_property<T: Append + Arg>(&self, prop: &str, value: T) -> Result<(), BoxError> {
        Ok(self
            .session
            .set_property(&self.path, ADAPTER_INTERFACE, prop, value)
            .await?)
    }

    //--------------------------------------------------------------------------------
    // プロパティ
    // get
    async_get_property!(get_address, String, "Address");
    async_get_property!(get_name, String, "Name");
    async_get_property!(get_alias, String, "Alias");
    async_get_property!(get_class, u32, "Class");
    async_get_property!(is_powered, bool, "Powered");
    async_get_property!(is_discoverable, bool, "Discoverable");
    async_get_property!(is_pairable, bool, "Pairable");
    async_get_property!(get_pairable_timeout, u32, "PairableTimeout");
    async_get_property!(get_discoverable_timeout, u32, "DiscoverableTimeout");
    async_get_property!(is_discovering, bool, "Discovering");
    async_get_property!(get_uuids, Vec<String>, "UUIDs");
    async_get_property!(get_modalias, String, "Modalias");
    // set
    async_set_property!(set_alias, String, "Alias");
    async_set_property!(set_powered, bool, "Powered");
    async_set_property!(set_discoverable, bool, "Discoverable");
    async_set_property!(set_pairable, bool, "Pairable");
    async_set_property!(set_pairable_timeout, u32, "PairableTimeout");
    async_set_property!(set_discoverable_timeout, u32, "DiscoverableTimeout");
}
