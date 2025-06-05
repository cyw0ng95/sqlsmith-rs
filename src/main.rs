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
use profile::read_profile;

fn main() -> Result<()> {
    utils::logger::init(); // Configure logging

    let profile = read_profile();
    let driver_kind = profile.driver.expect("driver kind must be specified");
    let run_count = profile.count.expect("run count must be an unsigned number");
    profile.print();

    // 修改: 传入 profile 参数
    let mut engine = Engine::with_driver_kind(0, driver_kind, run_count, &profile)?;
    info!("SQLite connection prepared and verified.");

    engine.run();

    Ok(())
}