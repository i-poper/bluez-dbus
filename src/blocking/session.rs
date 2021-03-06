use crate::*;
use dbus::arg::{Append, AppendAll, Arg, Get, ReadAll, Variant};
use dbus::blocking::Connection;
use std::fmt;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::time::Duration;

static MANAGED_OBJECT_INTERFACE: &str = "org.freedesktop.DBus.ObjectManager";
static MANAGED_OBJECT_METHOD: &str = "GetManagedObjects";

pub struct Session {
    // 複数スレッドでも使えるように`Mutex`を使用している
    // その分性能を犠牲にしている。
    conn: Arc<Mutex<Connection>>,
}

impl Debug for Session {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let conn = self.conn.lock().unwrap();
        write!(f, "Session {{ conn: {} }}", conn.unique_name())
    }
}

/// BlueZとの通信を行うセッション
impl Session {
    /// BlueZとの通信を行うセッションの作成
    pub fn new() -> Result<Self, BoxError> {
        let conn = Connection::new_system()?;
        Ok(Session {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    /// bluetoothアダプターの一覧を取得
    pub fn get_adapters(&self) -> Result<Option<Vec<String>>, BoxError> {
        let objects = self.get_managed_objects()?;

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

    pub(in crate) fn get_property<A: for<'z> Get<'z>>(
        &self,
        path: &str,
        interface: &str,
        property: &str,
    ) -> Result<A, BoxError> {
        let (value,): (Variant<A>,) = self.method_call(
            path,
            "org.freedesktop.DBus.Properties",
            "Get",
            (interface, property.to_string()),
        )?;
        Ok(value.0)
    }

    pub(in crate) fn set_property<A: Append + Arg>(
        &self,
        path: &str,
        interface: &str,
        property: &str,
        value: A,
    ) -> Result<(), BoxError> {
        let value = Variant(value);
        let _ = self.method_call(
            path,
            "org.freedesktop.DBus.Properties",
            "Set",
            (interface, property.to_string(), value),
        )?;
        Ok(())
    }

    /// BlueZに対するメソッド実行
    pub(in crate) fn method_call<R: ReadAll, A: AppendAll>(
        &self,
        path: &str,
        interface: &str,
        method: &str,
        arg: A,
    ) -> Result<R, dbus::Error> {
        let conn = self.conn.lock().unwrap();
        let proxy = conn.with_proxy(BLUEZ_SERVICE, path, Duration::from_secs(10));
        proxy.method_call(interface, method, arg)
    }

    /// 指定のパス配下の子要素の一覧を取得
    pub(in crate) fn get_children(
        &self,
        path: &str,
        prop: &str,
    ) -> Result<Option<Vec<String>>, BoxError> {
        let objects = self.get_managed_objects()?;

        let devices: Vec<String> = objects
            .iter()
            .filter_map(|(key, value)| {
                if is_match(path, prop, value) {
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

    pub(in crate) fn get_managed_objects(&self) -> Result<ManagedObject, BoxError> {
        let (managed_objects,): (ManagedObject,) =
            self.method_call("/", MANAGED_OBJECT_INTERFACE, MANAGED_OBJECT_METHOD, ())?;
        Ok(managed_objects)
    }
}

fn is_match(path: &str, prop: &str, info: &ManagedObjectInterfaces) -> bool {
    info.iter().any(|(_key, value)| {
        if let Some(s) = value.get_str(prop) {
            return s == path;
        }
        false
    })
}
