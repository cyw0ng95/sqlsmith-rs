use serde::Deserialize;
use crate::drivers::DRIVER_KIND;
use std::fs;
use log::info;

#[derive(Debug, Deserialize)]
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
    info!("profile.json not found or invalid, using debug config: SQLITE_IN_MEM, count 8");
    Profile {
        driver: Some(DRIVER_KIND::SQLITE_IN_MEM),
        count: Some(8),
    }
}