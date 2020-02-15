use crate::nonblock::Session;
use crate::*;
use dbus::arg::Get;
use dbus::strings::Path;

static DESCRIPTOR_INTERFACE: &str = "org.bluez.GattDescriptor1";

#[derive(Debug)]
pub struct Descriptor {
    session: Session,
    path: String,
}

impl Descriptor {
    /// Descriptor作成
    pub fn new(session: &Session, path: &str) -> Self {
        Descriptor {
            session: session.clone(),
            path: path.to_string(),
        }
    }
    pub async fn read_value(&self) -> Result<Vec<u8>, BoxError> {
        let (value,): (Vec<u8>,) = self
            .session
            .method_call(&self.path, DESCRIPTOR_INTERFACE, "ReadValue", ())
            .await?;
        Ok(value)
    }

    pub async fn write_value(&self, values: Vec<u8>) -> Result<(), BoxError> {
        Ok(self
            .session
            .method_call(&self.path, DESCRIPTOR_INTERFACE, "WriteValue", (values,))
            .await?)
    }

    async fn get_property<A: for<'z> Get<'z> + 'static>(
        &self,
        property: &str,
    ) -> Result<A, BoxError> {
        Ok(self
            .session
            .get_property(&self.path, DESCRIPTOR_INTERFACE, property)
            .await?)
    }

    //--------------------------------------------------------------------------------
    // プロパティ
    // get
    async_get_property!(get_uuid, String, "UUID");
    async_get_property!(get_characteristic, Path<'_>, "Characteristic");
    async_get_property!(get_value, Vec<u8>, "Value");
    async_get_property!(get_flags, Vec<String>, "Flags");
}
