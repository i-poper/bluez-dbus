use crate::nonblock::Session;
use crate::*;
use dbus::arg::Get;
use dbus::strings::Path;

static CHARACTERISTIC_INTERFACE: &str = "org.bluez.GattCharacteristic1";

#[derive(Debug)]
pub struct Characteristic {
    session: Session,
    path: String,
}

impl Characteristic {
    /// Gatt Service作成
    pub fn new(session: &Session, path: &str) -> Self {
        Characteristic {
            session: session.clone(),
            path: path.to_string(),
        }
    }

    pub async fn get_descriptors(&self) -> Result<Option<Vec<String>>, BoxError> {
        self.session
            .get_children(&self.path, "Characteristic")
            .await
    }

    pub async fn read_value(&self) -> Result<Vec<u8>, BoxError> {
        let (value,): (Vec<u8>,) = self
            .session
            .method_call(&self.path, CHARACTERISTIC_INTERFACE, "ReadValue", ())
            .await?;
        Ok(value)
    }

    pub async fn write_value(&self, values: Vec<u8>) -> Result<(), BoxError> {
        Ok(self
            .session
            .method_call(
                &self.path,
                CHARACTERISTIC_INTERFACE,
                "WriteValue",
                (values,),
            )
            .await?)
    }

    pub async fn start_notify(&self) -> Result<(), BoxError> {
        Ok(self
            .session
            .method_call(&self.path, CHARACTERISTIC_INTERFACE, "StartNotify", ())
            .await?)
    }

    pub async fn stop_notify(&self) -> Result<(), BoxError> {
        Ok(self
            .session
            .method_call(&self.path, CHARACTERISTIC_INTERFACE, "StopNotify", ())
            .await?)
    }

    async fn get_property<A: for<'z> Get<'z> + 'static>(
        &self,
        property: &str,
    ) -> Result<A, BoxError> {
        Ok(self
            .session
            .get_property(&self.path, CHARACTERISTIC_INTERFACE, property)
            .await?)
    }

    //--------------------------------------------------------------------------------
    // プロパティ
    // get
    async_get_property!(get_uuid, String, "UUID");
    async_get_property!(get_service, Path<'_>, "Service");
    async_get_property!(is_notifying, bool, "Notifying");
    // TODO: Flags
    // TODO: Descriptors
}
