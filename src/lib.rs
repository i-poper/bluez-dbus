use dbus::arg;
use dbus::arg::RefArg;
use std::collections::HashMap;
use std::error::Error;

pub mod blocking;

type ManagedObjectInterfaces =
    HashMap<String, HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>>;
type ManagedObject = HashMap<dbus::Path<'static>, ManagedObjectInterfaces>;

type BoxError = Box<dyn Error + 'static>;

static BLUEZ_SERVICE: &str = "org.bluez";

/// BlueZの`managed object`から値を取得する
trait TypeUtil {
    fn get_str(&self, key: &str) -> Option<String>;
}

impl TypeUtil for HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>> {
    fn get_str(&self, key: &str) -> Option<String> {
        if let Some(value) = self.get(key) {
            if let Some(s) = value.as_str() {
                return Some(s.to_string());
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
