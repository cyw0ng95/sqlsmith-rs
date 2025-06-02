// src/main.rs

// Declare the drivers module so Rust can find its content
mod drivers;
mod utils;

use drivers::DatabaseDriver; // Import the trait for type hinting and polymorphism
use anyhow::Result;
use log::info; // <-- Add this

fn main() -> Result<()> {
    utils::logger::init(); // Configure logging
    // --- SQLite Example ---
    info!("--- SQLite Driver ---"); // changed
    let sqlite_driver = drivers::sqlite_in_mem::SqliteDriver::new(":memory:"); // Or "my_database.db"
    let mut sqlite_conn = sqlite_driver.connect()?; // `connect()` is sync
    info!("SQLite connection object obtained."); // changed

    sqlite_driver.init(&mut sqlite_conn)?; // `init()` is sync
    info!("SQLite database initialized (TPC-C tables created)."); // changed

    // Optional: Verify a table in SQLite
    let count: i32 = sqlite_conn.query_row(
        "SELECT count(*) FROM warehouse",
        rusqlite::params![], // Use rusqlite's params! macro for empty parameters
        |row| row.get(0)
    )?;
    info!("Number of rows in 'warehouse' table (SQLite): {}\n", count); // changed

    Ok(())
}