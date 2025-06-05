use anyhow::Result;
use serde::{Deserialize, Serialize};

pub mod sqlite_in_mem;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DRIVER_KIND {
    SQLITE_IN_MEM,
    LIMBO, // 新增 LIMBO 类型
}

pub trait DatabaseDriver {
    /// The associated type for the database connection object.
    type Connection;

    fn connect(&self) -> Result<Self::Connection>;
    fn init(&self, conn: &mut Self::Connection) -> Result<()>;
    
    // 新增: 区分执行和查询
    fn exec(&self, conn: &mut Self::Connection, sql: &str) -> Result<usize>;
    fn query(&self, conn: &mut Self::Connection, sql: &str) -> Result<usize>;
}

/// 通用接口：根据 DRIVER_KIND 创建驱动和连接
pub fn new_conn(
    kind: DRIVER_KIND,
) -> Result<(Box<dyn DatabaseDriver<Connection = rusqlite::Connection>>, Box<rusqlite::Connection>)> {
    match kind {
        DRIVER_KIND::SQLITE_IN_MEM => {
            let driver = sqlite_in_mem::SqliteDriver::new();
            let mut conn = Box::new(driver.connect()?);
            driver.init(&mut conn)?;
            Ok((Box::new(driver), conn))
        }
        DRIVER_KIND::LIMBO => {
            // LIMBO 返回一个空实现和 in-memory rusqlite::Connection
            struct LimboDriver;
            impl DatabaseDriver for LimboDriver {
                type Connection = rusqlite::Connection;
                fn connect(&self) -> Result<Self::Connection> {
                    anyhow::bail!("LIMBO driver does not support connect()");
                }
                fn init(&self, _conn: &mut Self::Connection) -> Result<()> {
                    anyhow::bail!("LIMBO driver does not support init()");
                }
                fn exec(&self, _conn: &mut Self::Connection, _sql: &str) -> Result<usize> {
                    anyhow::bail!("LIMBO driver does not support exec()");
                }
                fn query(&self, _conn: &mut Self::Connection, _sql: &str) -> Result<usize> {
                    anyhow::bail!("LIMBO driver does not support query()");
                }
            }
            let driver = LimboDriver;
            let conn = Box::new(rusqlite::Connection::open_in_memory()?);
            Ok((Box::new(driver), conn))
        }
    }
}