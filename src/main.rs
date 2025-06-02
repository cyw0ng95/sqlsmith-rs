// src/main.rs

// Declare the drivers module so Rust can find its content
mod drivers;
mod utils;
mod generators;
mod engines;

use anyhow::Result;
use log::{info, error};
use engines::Engine;
use drivers::{new_conn, DRIVER_KIND};

fn main() -> Result<()> {
    utils::logger::init(); // Configure logging

    let (driver, mut sqlite_conn) = match new_conn(DRIVER_KIND::SQLITE_IN_MEM) {
        Ok((driver, conn)) => (driver, conn),
        Err(e) => {
            error!("Error preparing SQLite connection: {:?}", e);
            return Err(e);
        }
    };
    info!("SQLite connection prepared and verified.");

    let mut engine = Engine::new(0, driver.as_ref());

    let mut i = 0;
    while i < 8 {
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