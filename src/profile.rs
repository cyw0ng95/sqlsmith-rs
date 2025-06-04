use serde::Deserialize;
use crate::drivers::DRIVER_KIND;
use std::fs;
use log::info;
use serde_json;

#[derive(Debug, Deserialize, serde::Serialize)] // 添加 Serialize 派生宏
pub struct Profile {
    pub driver: Option<DRIVER_KIND>,
    pub count: Option<usize>,
}

impl Profile {
    pub fn print(&self) {
        log::info!(
            "Profile: driver={:?}, count={:?}",
            self.driver,
            self.count
        );
    }
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
    };
    if let Ok(json_str) = serde_json::to_string_pretty(&default_profile) {
        if let Err(e) = fs::write("profile.json", json_str) {
            eprintln!("Failed to write profile.json: {}", e);
        }
    }
    std::process::exit(0);
}