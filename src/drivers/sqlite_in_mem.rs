// src/drivers/sqlite.rs

use super::DatabaseDriver; // Import the trait from the parent module
use anyhow::{Result, Context};
use log::info; // Import the log::info macro
use rusqlite::Connection;
use std::fs;
use std::path::Path;

/// Represents a SQLite database driver.
pub struct SqliteDriver {
    pub db_path: String, // Path to the SQLite database file (e.g., ":memory:")
}

impl SqliteDriver {
    /// Creates a new `SqliteDriver` instance for in-memory database.
    pub fn new() -> Self {
        SqliteDriver { db_path: ":memory:".to_string() }
    }

    pub fn connect(&self) -> Result<Connection> {
        info!("(SQLite) Attempting to connect to: {}", self.db_path);
        let conn = Connection::open(&self.db_path)
            .map_err(|e| anyhow::anyhow!("Failed to open SQLite database at '{}': {}", self.db_path, e))?;

        conn.execute("PRAGMA foreign_keys = ON;", ())
            .map_err(|e| anyhow::anyhow!("Failed to enable foreign keys for SQLite: {}", e))?;
        info!("(SQLite) Foreign key enforcement enabled.");

        Ok(conn)
    }

    pub fn init(&self, conn: &mut Connection) -> Result<()> {
        info!("(SQLite) Executing init SQL from assets/sqlite/tpcc-create-table.sql...");
        let sql_file_path = Path::new("assets/sqlite/tpcc-create-table.sql");
        let sql_content = fs::read_to_string(sql_file_path)
            .map_err(|e| anyhow::anyhow!("Failed to read SQL file: {:?}: {}", sql_file_path, e))?;

        conn.execute_batch(&sql_content)
            .map_err(|e| anyhow::anyhow!("Failed to execute SQLite init SQL batch: {}", e))?;
        info!("(SQLite) TPC-C tables created successfully.");
        Ok(())
    }


    pub fn verify(&self, conn: &Connection) -> Result<bool> {
        let count: i32 = conn.query_row(
            "SELECT count(*) FROM warehouse",
            rusqlite::params![],
            |row| row.get(0)
        )?;
        if count != 0 {
            return Ok(false);
        }

        let insert_sql = "INSERT INTO warehouse (w_id, w_name, w_ytd, w_tax, w_street_1, w_street_2, w_city, w_state, w_zip) \
                          VALUES (1, 'test', 0, 0, 'a', 'b', 'c', 'd', 'e')";
        conn.execute(insert_sql, rusqlite::params![])?;

        let (count, name): (i32, String) = conn.query_row(
            "SELECT count(*), w_name FROM warehouse",
            rusqlite::params![],
            |row| Ok((row.get(0)?, row.get(1)?))
        )?;
        if count != 1 || name != "test" {
            conn.execute("DELETE FROM warehouse WHERE w_id=1", rusqlite::params![])?;
            return Ok(false);
        }

        conn.execute("DELETE FROM warehouse WHERE w_id=1", rusqlite::params![])?;

        let count: i32 = conn.query_row(
            "SELECT count(*) FROM warehouse",
            rusqlite::params![],
            |row| row.get(0)
        )?;
        Ok(count == 0)
    }

    pub fn prepare(&self) -> anyhow::Result<Connection> {
        let mut conn = self.connect()?;
        self.init(&mut conn)?;
        let ok = self.verify(&conn)?;
        if !ok {
            anyhow::bail!("SQLite verify failed after init.");
        }
        Ok(conn)
    }
}

// Implement the DatabaseDriver trait for SqliteDriver
impl DatabaseDriver for SqliteDriver {
    // Define the associated connection type for SQLite
    type Connection = Connection; // rusqlite::Connection is synchronous

    fn connect(&self) -> Result<Self::Connection> {
        info!("(SQLite) Attempting to connect to: {}", self.db_path);
        let conn = Connection::open(&self.db_path)
            .with_context(|| format!("Failed to open SQLite database at '{}'", self.db_path))?;

        // Enable foreign key constraints for the current connection.
        // This is crucial for TPC-C schema if you expect FKs to be enforced.
        conn.execute("PRAGMA foreign_keys = ON;", ())
            .context("Failed to enable foreign keys for SQLite")?;
        info!("(SQLite) Foreign key enforcement enabled.");

        Ok(conn)
    }

    fn init(&self, conn: &mut Self::Connection) -> Result<()> {
        info!("(SQLite) Executing init SQL from assets/sqlite/tpcc-create-table.sql...");
        let sql_file_path = Path::new("assets/sqlite/tpcc-create-table.sql");
        let sql_content = fs::read_to_string(sql_file_path)
            .with_context(|| format!("Failed to read SQL file: {:?}", sql_file_path))?;

        conn.execute_batch(&sql_content)
            .context("Failed to execute SQLite init SQL batch")?;
        info!("(SQLite) TPC-C tables created successfully.");
        Ok(())
    }

    fn exec(&self, conn: &mut Connection, sql: &str) -> anyhow::Result<usize> {
        Ok(conn.execute(sql, rusqlite::params![])?)
    }
}