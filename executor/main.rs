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

    // 修改：直接调用 with_driver_kind 函数
    let mut engine = with_driver_kind(0, driver_kind, run_count, &profile)?;
    info!("SQLite connection prepared and verified.");

    engine.run();

    Ok(())
}