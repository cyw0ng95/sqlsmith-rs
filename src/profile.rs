use serde::{Deserialize, Serialize};
use std::fs;
use log::info;
use crate::drivers::DRIVER_KIND;

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub driver: Option<DRIVER_KIND>,
    pub count: Option<usize>,
    pub stmt_prob: Option<StmtProb>, // 新增字段
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StmtProb {
    pub SELECT: u64,
    pub INSERT: u64,
    pub UPDATE: u64,
    pub VACUUM: u64, // 新增 VACUUM 字段
}

pub fn read_profile() -> Profile {
    if let Ok(content) = fs::read_to_string("profile.json") {
        if let Ok(profile) = serde_json::from_str::<Profile>(&content) {
            return profile;
        }
    }
    info!("profile.json not found or invalid, creating default profile.json and exiting.");
    let default_profile = Profile {
        driver: Some(DRIVER_KIND::SQLITE_IN_MEM),
        count: Some(8),
        stmt_prob: Some(StmtProb {
            SELECT: 100,
            INSERT: 50,
            UPDATE: 50,
            VACUUM: 20, // 新增默认值
        }),
    };
    if let Ok(json_str) = serde_json::to_string_pretty(&default_profile) {
        if let Err(e) = fs::write("profile.json", json_str) {
            eprintln!("Failed to write profile.json: {}", e);
        }
    }
    std::process::exit(0);
}

impl Profile {
    pub fn print(&self) {
        info!("Profile Information:");
        if let Some(driver) = self.driver {
            info!("Driver: {:?}", driver);
        } else {
            info!("Driver: Not specified");
        }
        if let Some(count) = self.count {
            info!("Run Count: {}", count);
        } else {
            info!("Run Count: Not specified");
        }
        if let Some(stmt_prob) = &self.stmt_prob {
            info!("Statement Probabilities:");
            info!("  SELECT: {}", stmt_prob.SELECT);
            info!("  INSERT: {}", stmt_prob.INSERT);
            info!("  UPDATE: {}", stmt_prob.UPDATE);
            info!("  VACUUM: {}", stmt_prob.VACUUM); // 新增打印信息
        } else {
            info!("Statement Probabilities: Not specified");
        }
    }
}