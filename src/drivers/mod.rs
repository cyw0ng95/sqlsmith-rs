use anyhow::Result;
use serde::{Deserialize, Serialize};

pub mod sqlite_in_mem;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DRIVER_KIND {
    SQLITE_IN_MEM,
}

pub trait DatabaseDriver {
    /// The associated type for the database connection object.
    type Connection;

    fn connect(&self) -> Result<Self::Connection>;
    fn init(&self, conn: &mut Self::Connection) -> Result<()>;
    fn exec(&self, conn: &mut Self::Connection, sql: &str) -> Result<usize>;
}

/// 通用接口：根据 DRIVER_KIND 创建驱动和连接
pub fn new_conn(
    kind: DRIVER_KIND,
) -> Result<(Box<dyn DatabaseDriver<Connection = rusqlite::Connection>>, rusqlite::Connection)> {
    match kind {
        DRIVER_KIND::SQLITE_IN_MEM => {
            let driver = sqlite_in_mem::SqliteDriver::new();
            let mut conn = driver.connect()?;
            driver.init(&mut conn)?;
            Ok((Box::new(driver), conn))
        }
    }
}