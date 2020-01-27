use dbus::blocking::Connection;
use std::error::Error;

pub struct Session {
    conn: Connection,
}

/// BlueZとの通信を行うセッション
impl Session {

    /// BlueZとの通信を行うセッションの作成
    pub fn new() -> Result<Self,Box<dyn Error+'static>> {
        let conn = Connection::new_system()?;
        Ok(Session { conn })
    }

    /// BlueZとの通信を行うコネクションを取得
    pub fn get_connection(&self) -> &Connection {
        &self.conn
    }
}
