use crate::blocking::Session;
use crate::*;
use dbus::arg::Get;
use dbus::strings::Path;

static GATT_SERVICE_INTERFACE: &str = "org.bluez.GattService1";

#[derive(Debug)]
pub struct GattService<'a> {
    session: &'a Session,
    path: String,
}

impl<'a> GattService<'a> {
    /// Gatt Service作成
    pub fn new(session: &'a Session, path: &str) -> Self {
        GattService {
            session,
            path: path.to_string(),
        }
    }

    /// Gatt Serviceに属するCharacteristicの一覧を取得
    pub fn get_characteristics(&self) -> Result<Option<Vec<String>>, BoxError> {
        self.session.get_children(&self.path, "Service")
    }

    fn get_property<A: for<'z> Get<'z>>(&self, property: &str) -> Result<A, BoxError> {
        Ok(self
            .session
            .get_property(&self.path, GATT_SERVICE_INTERFACE, property)?)
    }

    //--------------------------------------------------------------------------------
    // プロパティ
    // get
    get_property!(get_uuid, String, "UUID");
    get_property!(is_primary, bool, "Primary");
    get_property!(get_device, Path, "Device");
    // get_property!(get_characteristics, Vec<String>, "Characteristics");
    get_property!(get_includes, Vec<Path>, "Includes");
}
