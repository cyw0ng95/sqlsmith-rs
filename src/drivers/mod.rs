use anyhow::Result;
use serde::{Deserialize, Serialize};

pub mod sqlite_in_mem;
pub mod limbo_in_mem; // <-- 添加这一行

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DRIVER_KIND {
    SQLITE_IN_MEM,
    LIMBO_IN_MEM, // 新增 LIMBO 类型
}

pub trait DatabaseDriver {
    /// The associated type for the database connection object.
    type Connection;

    fn exec(&self, sql: &str) -> Result<usize>;
    fn query(&self, sql: &str) -> Result<usize>;
    fn get_connection(&self) -> &Self::Connection;
    fn get_connection_mut(&mut self) -> &mut Self::Connection;
}

/// 通用接口：根据 DRIVER_KIND 创建驱动和连接
pub fn new_conn(
    kind: DRIVER_KIND,
) -> Result<Box<dyn DatabaseDriver<Connection = rusqlite::Connection>>> {
    match kind {
        DRIVER_KIND::SQLITE_IN_MEM => {
            let driver = sqlite_in_mem::SqliteDriver::new()?;
            Ok(Box::new(driver))
        }
        DRIVER_KIND::LIMBO_IN_MEM => {
            anyhow::bail!("LIMBO driver is not implemented")
        }
    }
}