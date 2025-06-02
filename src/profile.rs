use serde::Deserialize;
use crate::drivers::DRIVER_KIND;
use std::fs;
use log::info;

#[derive(Debug, Deserialize)]
pub struct Profile {
    pub driver: Option<DRIVER_KIND>,
    pub count: Option<usize>,
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

pub fn profile_to_config(profile: &Profile) -> (DRIVER_KIND, usize) {
    let kind = profile.driver.unwrap_or(DRIVER_KIND::SQLITE_IN_MEM);
    let count = profile.count.unwrap_or(8);
    (kind, count)
}