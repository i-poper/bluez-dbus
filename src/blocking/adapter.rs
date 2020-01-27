use crate::blocking::Session;
use crate::*;
use std::error::Error;

static ADAPTER_INTERFACE: &str = "org.bluez.Adapter1";

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
        if let Some(adapters) = Adapter::list(session)? {
            if adapters.contains(&path.to_string()) {
                return Ok(Some(Adapter::new(session, path)));
            }
        }
        Ok(None)
    }

    /// bluetoothアダプターの一覧を取得
    pub fn list(session: &'a Session) -> Result<Option<Vec<String>>, BoxError> {
        let objects = session.get_managed_objects()?;

        let adapters: Vec<String> = objects
            .iter()
            .filter_map(|(key, value)| {
                if value.contains_key(ADAPTER_INTERFACE) {
                    return Some(key.to_string());
                }
                None
            })
            .collect();

        if adapters.is_empty() {
            Ok(None)
        } else {
            Ok(Some(adapters))
        }
    }

    /// デバイスリスト取得
    ///
    /// アダプターに登録されているデバイスのパスのリストを取得する
    pub fn device_list(&self) -> Result<Option<Vec<String>>, BoxError> {
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

    fn sub_discovery(&self, method: &str) -> Result<(), BoxError> {
        self.session
            .method_call(&self.path, ADAPTER_INTERFACE, method, ())?;
        Ok(())
    }
}
