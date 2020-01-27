use crate::blocking::Session;
use dbus::arg;
use dbus::arg::RefArg;
use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;

static ADAPTER_INTERFACE: &str = "org.bluez.Adapter1";
static BLUEZ_SERVICE: &str = "org.bluez";
static MANAGED_OBJECT_INTERFACE: &str = "org.freedesktop.DBus.ObjectManager";
static MANAGED_OBJECT_METHOD: &str = "GetManagedObjects";

type ManagedObject = HashMap<
    dbus::Path<'static>,
    HashMap<String, HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>>,
>;

type BoxError = Box<dyn Error + 'static>;

/// BlueZの`managed object`から値を取得する
trait TypeUtil {
    fn get_as_str(&self, key: &str) -> Option<String>;
}

impl TypeUtil for HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>> {
    fn get_as_str(&self, key: &str) -> Option<String> {
        if let Some(value) = self.get(key) {
            if let Some(s) = value.as_str() {
                return Some(s.to_string());
            }
        }
        None
    }
}

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
        let objects = get_managed_objects(session)?;

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
        let objects = get_managed_objects(self.session)?;

        let devices: Vec<String> = objects
            .iter()
            .filter_map(|(key, value)| {
                if is_adapter_device(&self.path, value) {
                    Some(key.to_string())
                } else {
                    None
                }
            })
            .collect();
        if devices.is_empty() {
            Ok(None)
        } else {
            Ok(Some(devices))
        }
    }
}

fn is_adapter_device(
    path: &str,
    info: &HashMap<String, HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>>,
) -> bool {
    info.iter().any(|(_key, value)| {
        if let Some(s) = value.get_as_str("Adapter") {
            return s == path;
        }
        false
    })
}

fn get_managed_objects(session: &Session) -> Result<ManagedObject, BoxError> {
    let proxy = session
        .get_connection()
        .with_proxy(BLUEZ_SERVICE, "/", Duration::from_secs(10));
    let (managed_objects,): (ManagedObject,) =
        proxy.method_call(MANAGED_OBJECT_INTERFACE, MANAGED_OBJECT_METHOD, ())?;
    Ok(managed_objects)
}
