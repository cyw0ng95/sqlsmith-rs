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

    let sqlite_driver: SqliteDriver = SqliteDriver::new();
    let mut sqlite_conn = match sqlite_driver.prepare() {
        Ok(conn) => conn,
        Err(e) => {
            error!("Error preparing SQLite connection: {:?}", e);
            return Err(e);
        }
    };
    info!("SQLite connection prepared and verified.");

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