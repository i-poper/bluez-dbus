use crate::*;
use dbus::arg::{Append, AppendAll, Arg, Get, ReadAll, Variant};
#[cfg(feature = "local")]
use dbus::nonblock::LocalConnection;
use dbus::nonblock::Proxy;
#[cfg(not(feature = "local"))]
use dbus::nonblock::SyncConnection;
use dbus_tokio::connection as dbus_conn;
use std::fmt;
use std::fmt::Debug;
use std::sync::Arc;
use std::time::Duration;
use tokio::task;

static MANAGED_OBJECT_INTERFACE: &str = "org.freedesktop.DBus.ObjectManager";
static MANAGED_OBJECT_METHOD: &str = "GetManagedObjects";

/// BlueZとのセッション
#[derive(Clone)]
pub struct Session {
    #[cfg(not(feature = "local"))]
    conn: Arc<SyncConnection>,
    #[cfg(feature = "local")]
    conn: Arc<LocalConnection>,
}

impl Debug for Session {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Session {{ conn: {} }}", self.conn.unique_name())
    }
}

/// BlueZとの通信を行うセッション
impl Session {
    /// BlueZとの通信を行うセッションの作成
    pub fn new() -> Result<Self, BoxError> {
        #[cfg(not(feature = "local"))]
        let conn = {
            let (resource, conn) = dbus_conn::new_system_sync()?;
            task::spawn(async move {
                let err = resource.await;
                panic!("Lost connection to D-Bus: {}", err);
            });
            conn
        };
        #[cfg(feature = "local")]
        let conn = {
            let (resource, conn) = dbus_conn::new_system_local()?;
            task::spawn_local(async move {
                let err = resource.await;
                panic!("Lost connection to D-Bus: {}", err);
            });
            conn
        };

        Ok(Session { conn })
    }

    /// bluetoothアダプターの一覧を取得
    pub async fn get_adapters(&self) -> Result<Option<Vec<String>>, BoxError> {
        let objects = self.get_managed_objects().await?;

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

    pub(in crate) async fn get_property<A: for<'z> Get<'z> + 'static>(
        &self,
        path: &str,
        interface: &str,
        property: &str,
    ) -> Result<A, BoxError> {
        let (value,): (Variant<A>,) = self
            .method_call(
                path,
                "org.freedesktop.DBus.Properties",
                "Get",
                (interface, property.to_string()),
            )
            .await?;
        Ok(value.0)
    }

    pub(in crate) async fn set_property<A: Append + Arg>(
        &self,
        path: &str,
        interface: &str,
        property: &str,
        value: A,
    ) -> Result<(), BoxError> {
        let value = Variant(value);
        let _ = self
            .method_call(
                path,
                "org.freedesktop.DBus.Properties",
                "Set",
                (interface, property.to_string(), value),
            )
            .await?;
        Ok(())
    }

    /// BlueZに対するメソッド実行
    pub(in crate) async fn method_call<R: ReadAll + 'static, A: AppendAll>(
        &self,
        path: &str,
        interface: &str,
        method: &str,
        arg: A,
    ) -> Result<R, dbus::Error> {
        let conn = self.conn.clone();
        let proxy = Proxy::new(BLUEZ_SERVICE, path, Duration::from_secs(10), conn);
        proxy.method_call(interface, method, arg).await
    }

    /// 指定のパス配下の子要素の一覧を取得
    pub(in crate) async fn get_children(
        &self,
        path: &str,
        prop: &str,
    ) -> Result<Option<Vec<String>>, BoxError> {
        let objects = self.get_managed_objects().await?;

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

    pub(in crate) async fn get_managed_objects(&self) -> Result<ManagedObject, BoxError> {
        let (managed_objects,): (ManagedObject,) = self
            .method_call("/", MANAGED_OBJECT_INTERFACE, MANAGED_OBJECT_METHOD, ())
            .await?;
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
