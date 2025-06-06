use anyhow::Result;
use limbo::{Builder, Connection};
use crate::drivers::DatabaseDriver;

pub struct LimboDriver {
    conn: Connection,
}

impl LimboDriver {
    pub async fn new() -> Result<Self> {
        let db = Builder::new_local("sqlite.db").build().await?;
        let conn = db.connect()?;
        Ok(Self { conn })
    }
}

impl DatabaseDriver for LimboDriver {
    // Use actual connection type instead of `()`
    type Connection = limbo::Connection;

    fn exec(&self, sql: &str) -> Result<usize> {
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(async {
            let result = self.conn.execute(sql, ()).await?;
            Ok(result as usize)
        })
    }

    fn query(&self, sql: &str) -> Result<usize> {
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(async {
            let mut rows = self.conn.query(sql, ()).await?;
            let mut count = 0;
            while let Ok(Some(_)) = rows.next().await {
                count += 1;
            }
            Ok(count)
        })
    }

    // Implement connection accessors with the correct type
    fn get_connection(&self) -> &Self::Connection {
        &self.conn
    }
    fn get_connection_mut(&mut self) -> &mut Self::Connection {
        &mut self.conn
    }
}