// src/drivers/sqlite.rs

use super::DatabaseDriver; // Import the trait from the parent module
use rusqlite::Connection;
use anyhow::{Result, Context};
use std::fs;
use std::path::Path;
use log::info; // Import the log::info macro

/// Represents a SQLite database driver.
pub struct SqliteDriver {
    db_path: String, // Path to the SQLite database file (e.g., ":memory:" or "my_db.db")
}

impl SqliteDriver {
    /// Creates a new `SqliteDriver` instance.
    ///
    /// # Arguments
    /// * `db_path` - The path to the SQLite database file. Use ":memory:" for an in-memory database.
    pub fn new(db_path: &str) -> Self {
        SqliteDriver { db_path: db_path.to_string() }
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
}