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
    pub DELETE: u64,
    pub SELECT: u64,
    pub INSERT: u64,
    pub UPDATE: u64,
    pub UPSERT: u64,
    pub VACUUM: u64,
    pub PRAGMA: u64,
}

pub fn read_profile() -> Profile {
    if let Ok(content) = fs::read_to_string("profile.json") {
        if let Ok(profile) = serde_json::from_str::<Profile>(&content) {
            return profile;
        }
    }

    // 直接使用默认值生成 Profile 结构体
    let driver = Some(DRIVER_KIND::SQLITE_IN_MEM);
    let count = Some(8);
    let executor_count = Some(5);
    let stmt_prob = Some(StmtProb {
        SELECT: 100,
        INSERT: 50,
        UPDATE: 50,
        UPSERT: 30,
        DELETE: 20,
        VACUUM: 20,
        PRAGMA: 10,
    });
    let debug = Some(DebugOptions {
        show_success_sql: false,
        show_failed_sql: true,
    });
    let seed = Some(0);

    let profile = Profile {
        driver,
        count,
        executor_count,
        stmt_prob,
        debug,
        seed,
    };

    if let Ok(json_str) = serde_json::to_string_pretty(&profile) {
        if let Err(e) = fs::write("profile.json", json_str) {
            eprintln!("Failed to write profile.json: {}", e);
        }
    }
    info!("default profile.json created");
    profile
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
            items.push(format!("DELETE={}", stmt_prob.DELETE));
            items.push(format!("PRAGMA={}", stmt_prob.PRAGMA)); // 新增
        }
        if let Some(debug) = &self.debug {
            items.push(format!("show_success_sql={}", debug.show_success_sql));
            items.push(format!("show_failed_sql={}", debug.show_failed_sql));
        }
        log::info!("Profile: {}", items.join(", "));
    }
}