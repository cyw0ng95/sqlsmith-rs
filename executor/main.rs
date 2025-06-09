// src/main.rs

// Declare the drivers module so Rust can find its content
mod generators;
mod engines;

use anyhow::Result;
use log::info;
use engines::Engine;
use sqlsmith_rs_common::profile::read_profile;

use crate::engines::with_driver_kind;

fn main() -> Result<()> {
    sqlsmith_rs_common::logger::init(); // Configure logging

    let profile = read_profile();
    let driver_kind = profile.driver.expect("driver kind must be specified");
    let run_count = profile.count.expect("run count must be an unsigned number");
    profile.print();

    // Get seed from environment variable or fallback to profile seed
    let seed = std::env::var("EXEC_PARAM_SEED")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| profile.seed.unwrap_or(0));
    info!("init executor engine with seed: {:?}, base seed: {:?}", seed, profile.seed.unwrap());
    let mut engine = with_driver_kind(seed, driver_kind, run_count, &profile)?;
    info!("SQLite connection prepared and verified.");

    engine.run();

    Ok(())
}