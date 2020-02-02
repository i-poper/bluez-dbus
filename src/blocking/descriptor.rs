use crate::blocking::Session;
use crate::*;
use dbus::arg::Get;
use dbus::strings::Path;

static DESCRIPTOR_INTERFACE: &str = "org.bluez.GattDescriptor1";

#[derive(Debug)]
pub struct Descriptor<'a> {
    session: &'a Session,
    path: String,
}

impl<'a> Descriptor<'a> {
    /// Descriptor作成
    pub fn new(session: &'a Session, path: &str) -> Self {
        Descriptor {
            session,
            path: path.to_string(),
        }
    }
    pub fn read_value(&self) -> Result<Vec<u8>, BoxError> {
        let (value,): (Vec<u8>,) =
            self.session
                .method_call(&self.path, DESCRIPTOR_INTERFACE, "ReadValue", ())?;
        Ok(value)
    }

    pub fn write_value(&self, values: Vec<u8>) -> Result<(), BoxError> {
        Ok(self
            .session
            .method_call(&self.path, DESCRIPTOR_INTERFACE, "WriteValue", (values,))?)
    }

    fn get_property<A: for<'z> Get<'z>>(&self, property: &str) -> Result<A, BoxError> {
        Ok(self
            .session
            .get_property(&self.path, DESCRIPTOR_INTERFACE, property)?)
    }

    //--------------------------------------------------------------------------------
    // プロパティ
    // get
    get_property!(get_uuid, String, "UUID");
    get_property!(get_characteristic, Path, "Characteristic");
    get_property!(get_value, Vec<u8>, "Value");
    get_property!(get_flags, Vec<String>, "Flags");
}
