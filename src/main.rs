// src/main.rs

// Declare the drivers module so Rust can find its content
mod drivers;
mod utils;
mod generators;
mod engines;

use anyhow::Result;
use log::{info, error}; // <-- Add this
use engines::Engine;
use drivers::sqlite_in_mem::SqliteDriver;

fn main() -> Result<()> {
    utils::logger::init(); // Configure logging
    // --- SQLite Example ---
    info!("--- SQLite Driver ---"); // changed

    // SQLITE_IN_MEM 类型无需传递 ":memory:"
    let sqlite_driver = SqliteDriver::new();

    let mut sqlite_conn = sqlite_driver.connect()?; // `connect()` is sync
    info!("SQLite connection object obtained."); // changed

    sqlite_driver.init(&mut sqlite_conn)?; // `init()` is sync
    info!("SQLite database initialized (TPC-C tables created).");

    let ok = match sqlite_driver.verify(&sqlite_conn) {
        Ok(v) => v,
        Err(e) => {
            error!("Error verifying warehouse table: {:?}", e);
            return Err(e);
        }
    };
    info!("Verify warehouse table: {}", if ok { "OK" } else { "FAILED" });

    let mut engine = Engine::new(0, &sqlite_driver);

    let mut i = 0;
    while i < 1000 {
        let sql = engine.next_sql(&sqlite_conn)
            .unwrap_or_else(|| "SELECT 1;".to_string());
        info!("Generated SQL: {}", sql);

        let result = engine.exec(&mut sqlite_conn, &sql);
        match result {
            Ok(_) => {}
            Err(e) => {
                i += 1;
                info!("Error executing SQL with ret: [{:?}]", e);
                continue;
            }
        }

        i += 1;
    }
    Ok(())
}