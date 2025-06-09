use serde::{Deserialize, Serialize};
use std::fs;
use log::info;
use sqlsmith_rs_drivers::DRIVER_KIND;

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub driver: Option<DRIVER_KIND>,
    pub count: Option<usize>,
    pub executor_count: Option<usize>,
    pub stmt_prob: Option<StmtProb>,
    pub debug: Option<DebugOptions>,
    pub seed: Option<u64>, // Added seed field
}

#[derive(Serialize, Deserialize, Debug, Clone)]  // 添加 Clone
pub struct DebugOptions {
    pub show_success_sql: bool,
    pub show_failed_sql: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]  // 添加 Clone
pub struct StmtProb {
    pub SELECT: u64,
    pub INSERT: u64,
    pub UPDATE: u64,
    pub UPSERT: u64,
    pub VACUUM: u64,
    pub PRAGMA: u64,
}

pub fn read_profile() -> Profile {
    use std::io::{self, Write};

    if let Ok(content) = fs::read_to_string("profile.json") {
        if let Ok(profile) = serde_json::from_str::<Profile>(&content) {
            return profile;
        }
    }

    info!("profile.json not found or invalid, please input configuration interactively.");

    // 交互式输入
    fn prompt<T>(msg: &str, default: T) -> T
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
        T: std::fmt::Display,
    {
        print!("{} [{}]: ", msg, default);
        io::stdout().flush().ok();
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok();
        let input = input.trim();
        if input.is_empty() {
            return default;
        }
        input.parse().unwrap_or(default)
    }

    // DRIVER_KIND
    println!("Select driver kind: 1) SQLITE_IN_MEM  2) LIMBO");
    let driver = match prompt("Driver kind (1/2)", 1u32) {
        2 => Some(DRIVER_KIND::LIMBO_IN_MEM),
        _ => Some(DRIVER_KIND::SQLITE_IN_MEM),
    };

    // count
    let count = Some(prompt("Run count", 8usize));

    // executor_count
    let executor_count = Some(prompt("Executor count", 5usize));

    // stmt_prob
    let select = prompt("SELECT probability", 100u64);
    let insert = prompt("INSERT probability", 50u64);
    let update = prompt("UPDATE probability", 50u64);
    let upsert = prompt("UPSERT probability", 30u64);
    let vacuum = prompt("VACUUM probability", 20u64);
    let pragma = prompt("PRAGMA probability", 10u64);
    let stmt_prob = Some(StmtProb {
        SELECT: select,
        INSERT: insert,
        UPDATE: update,
        UPSERT: upsert,
        VACUUM: vacuum,
        PRAGMA: pragma,
    });

    // debug options
    let show_success_sql = prompt("Show success SQL? (0/1)", 0u32) != 0;
    let show_failed_sql = prompt("Show failed SQL? (0/1)", 1u32) != 0;
    let debug = Some(DebugOptions {
        show_success_sql,
        show_failed_sql,
    });

    let profile = Profile {
        driver,
        count,
        executor_count,
        stmt_prob,
        debug,
        seed: Some(0), // Default seed value
    };

    if let Ok(json_str) = serde_json::to_string_pretty(&profile) {
        if let Err(e) = fs::write("profile.json", json_str) {
            eprintln!("Failed to write profile.json: {}", e);
        }
    }
    info!("profile.json created, please restart the program.");
    std::process::exit(0);
}

pub fn write_profile(profile: &Profile) -> Result<(), std::io::Error> {
    let json_str = serde_json::to_string_pretty(profile).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    fs::write("profile.json", json_str)?;
    Ok(())
}

impl Profile {
    pub fn print(&self) {
        let mut items = vec![];
        items.push(format!(
            "driver={:?}",
            self.driver.unwrap_or(DRIVER_KIND::SQLITE_IN_MEM)
        ));
        items.push(format!(
            "count={}",
            self.count.unwrap_or(8)
        ));
        items.push(format!(
            "executor_count={}",
            self.executor_count.unwrap_or(5)
        ));
        if let Some(seed) = self.seed {
            items.push(format!("seed={}", seed)); // Added seed to print
        }
        if let Some(stmt_prob) = &self.stmt_prob {
            items.push(format!("SELECT={}", stmt_prob.SELECT));
            items.push(format!("INSERT={}", stmt_prob.INSERT));
            items.push(format!("UPDATE={}", stmt_prob.UPDATE));
            items.push(format!("VACUUM={}", stmt_prob.VACUUM));
            items.push(format!("PRAGMA={}", stmt_prob.PRAGMA)); // 新增
        }
        if let Some(debug) = &self.debug {
            items.push(format!("show_success_sql={}", debug.show_success_sql));
            items.push(format!("show_failed_sql={}", debug.show_failed_sql));
        }
        log::info!("Profile: {}", items.join(", "));
    }
}