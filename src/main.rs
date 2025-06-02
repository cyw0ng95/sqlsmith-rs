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
    engine.run(&mut sqlite_conn, 8);

    Ok(())
}