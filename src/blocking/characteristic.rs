use crate::blocking::Session;
use crate::*;
use dbus::arg::Get;
use dbus::strings::Path;

static CHARACTERISTIC_INTERFACE: &str = "org.bluez.GattCharacteristic1";

#[derive(Debug)]
pub struct Characteristic<'a> {
    session: &'a Session,
    path: String,
}

impl<'a> Characteristic<'a> {
    /// Gatt Service作成
    pub fn new(session: &'a Session, path: &str) -> Self {
        Characteristic {
            session,
            path: path.to_string(),
        }
    }
    pub fn get_descriptors(&self) -> Result<Option<Vec<String>>, BoxError> {
        self.session.get_children(&self.path, "Characteristic")
    }

    pub fn read_value(&self) -> Result<Vec<u8>, BoxError> {
        let (value,): (Vec<u8>,) =
            self.session
                .method_call(&self.path, CHARACTERISTIC_INTERFACE, "ReadValue", ())?;
        Ok(value)
    }

    pub fn write_value(&self, values: Vec<u8>) -> Result<(), BoxError> {
        Ok(self.session
                .method_call(&self.path, CHARACTERISTIC_INTERFACE, "WriteValue", (values,))?)
    }

    pub fn start_notify(&self) -> Result<(), BoxError> {
        Ok(self.session
                .method_call(&self.path, CHARACTERISTIC_INTERFACE, "StartNotify", ())?)
    }

    pub fn stop_notify(&self) -> Result<(), BoxError> {
        Ok(self.session
                .method_call(&self.path, CHARACTERISTIC_INTERFACE, "StopNotify", ())?)
    }

    fn get_property<A: for<'z> Get<'z>>(&self, property: &str) -> Result<A, BoxError> {
        Ok(self
            .session
            .get_property(&self.path, CHARACTERISTIC_INTERFACE, property)?)
    }

    //--------------------------------------------------------------------------------
    // プロパティ
    // get
    get_property!(get_uuid, String, "UUID");
    get_property!(get_service, Path, "Service");
    get_property!(is_notifying, bool, "Notifying");
    // TODO: Flags
    // TODO: Descriptors
}
