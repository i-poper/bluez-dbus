use crate::nonblock::Session;
use crate::*;
use dbus::arg::Get;
use dbus::strings::Path;

static GATT_SERVICE_INTERFACE: &str = "org.bluez.GattService1";

#[derive(Debug)]
pub struct GattService {
    session: Session,
    path: String,
}

impl GattService {
    /// Gatt Service作成
    pub fn new(session: &Session, path: &str) -> Self {
        GattService {
            session: session.clone(),
            path: path.to_string(),
        }
    }

    /// Gatt Serviceに属するCharacteristicの一覧を取得
    pub async fn get_characteristics(&self) -> Result<Option<Vec<String>>, BoxError> {
        self.session.get_children(&self.path, "Service").await
    }

    async fn get_property<A: for<'z> Get<'z> + 'static>(
        &self,
        property: &str,
    ) -> Result<A, BoxError> {
        Ok(self
            .session
            .get_property(&self.path, GATT_SERVICE_INTERFACE, property)
            .await?)
    }

    //--------------------------------------------------------------------------------
    // プロパティ
    // get
    async_get_property!(get_uuid, String, "UUID");
    async_get_property!(is_primary, bool, "Primary");
    async_get_property!(get_device, Path<'_>, "Device");
    // get_property!(get_characteristics, Vec<String>, "Characteristics");
    async_get_property!(get_includes, Vec<Path<'_>>, "Includes");
}
