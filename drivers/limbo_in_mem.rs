use super::DatabaseDriver;
use anyhow::Result;
use limbo::{Builder, Connection};
use log::info;
use std::fs;
use std::path::Path;

pub struct LimboDriver {
    conn: Connection,
}

impl LimboDriver {
    pub async fn new() -> Result<Self> {
        let db = Builder::new_local(":memory:").build().await?;
        let conn = db.connect()?;
        let driver = Self { conn };

        // Initialize the database
        info!("Initializing Limbo database...");
        driver.init().await?;

        // TODO: Add verify logic similar to SQLite if needed

        Ok(driver)
    }

    async fn init(&self) -> Result<()> {
        info!("(Limbo) Executing init SQL from assets/limbo/tpcc-create-table.sql...");
        let sql_file_path = Path::new("assets/limbo/tpcc-create-table.sql");
        let sql_content = fs::read_to_string(sql_file_path)
            .map_err(|e| anyhow::anyhow!("Failed to read SQL file: {:?}: {}", sql_file_path, e))?;

        self.conn
            .execute(&sql_content, ())
            .await
            .map_err(|e| anyhow::anyhow!("Failed to execute Limbo init SQL batch: {}", e))?;
        info!("(Limbo) TPC-C tables created successfully.");
        Ok(())
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
