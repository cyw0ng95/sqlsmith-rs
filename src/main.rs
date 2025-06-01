// src/main.rs

// Declare the drivers module so Rust can find its content
mod drivers;

use drivers::DatabaseDriver; // Import the trait for type hinting and polymorphism
use anyhow::Result;

fn main() -> Result<()> {
    // --- SQLite Example ---
    println!("--- SQLite Driver ---");
    let sqlite_driver = drivers::sqlite_in_mem::SqliteDriver::new(":memory:"); // Or "my_database.db"
    let mut sqlite_conn = sqlite_driver.connect()?; // `connect()` is sync
    println!("SQLite connection object obtained.");

    sqlite_driver.init(&mut sqlite_conn)?; // `init()` is sync
    println!("SQLite database initialized (TPC-C tables created).");

    // Optional: Verify a table in SQLite
    let count: i32 = sqlite_conn.query_row(
        "SELECT count(*) FROM warehouse",
        rusqlite::params![], // Use rusqlite's params! macro for empty parameters
        |row| row.get(0)
    )?;
    println!("Number of rows in 'warehouse' table (SQLite): {}\n", count);

    Ok(())
}