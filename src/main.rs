// src/main.rs

// Declare the drivers module so Rust can find its content
mod drivers;
mod utils;
mod generators;
mod engines;
mod profile;

use anyhow::Result;
use log::{info, error};
use engines::Engine;
use drivers::{new_conn};
use profile::read_profile;

fn main() -> Result<()> {
    utils::logger::init(); // Configure logging

    let profile = read_profile();
    let driver_kind = profile.driver.expect("driver kind must be specified");
    let run_count = profile.count.expect("run count must be an unsigned number");
    profile.print();

    let (driver, mut sqlite_conn) = match new_conn(driver_kind) {
        Ok((driver, conn)) => (driver, conn),
        Err(e) => {
            error!("Error preparing SQLite connection: {:?}", e);
            return Err(e);
        }
    };
    info!("SQLite connection prepared and verified.");

    let mut engine = Engine::new(0, driver.as_ref());
    engine.run(&mut sqlite_conn, run_count);

    Ok(())
}