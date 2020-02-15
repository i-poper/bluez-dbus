mod session;
pub use session::Session;

mod adapter;
pub use adapter::Adapter;

mod device;
pub use device::Device;

mod gatt_service;
pub use gatt_service::GattService;

mod characteristic;
pub use characteristic::Characteristic;

mod descriptor;
pub use descriptor::Descriptor;

/// プロパティ取得の関数を作成するマクロ
#[doc(hidden)]
#[macro_export]
macro_rules! get_property {
    ($func: ident, $t: ty, $prop: expr) => {
        pub fn $func(&self) -> Result<$t, BoxError> {
            self.get_property($prop)
        }
    }
    }

/// プロパティ設定の関数を作成するマクロ
#[doc(hidden)]
#[macro_export]
macro_rules! set_property {
    ($func: ident, $t: ty, $prop: expr) => {
        pub fn $func(&self, value: $t) -> Result<(), BoxError> {
            self.set_property($prop, value)
        }
    }
}

