pub mod sqlite;

use rusqlite::Connection;
use crate::utils::rand_by_seed::LcgRng;
use serde_json::Value;
use std::fs::File;
use std::io::Read;

pub fn generate(conn: &Connection, rng: &mut LcgRng) -> String {
    let mut file = File::open("/home/cyw0ng/projects/sqlsmith-rs/profile.json").expect("Failed to open profile.json");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read profile.json");
    let json: Value = serde_json::from_str(&contents).expect("Failed to parse profile.json");

    let select_prob = json["stmt-prob"]["SELECT"].as_u64().unwrap_or(100);
    let insert_prob = json["stmt-prob"]["INSERT"].as_u64().unwrap_or(50);
    let total = select_prob + insert_prob;

    let random_num = rng.rand().unsigned_abs() as u64 % total;
    let sql_kind = if random_num < select_prob {
        sqlite::SQL_KIND::SELECT
    } else {
        sqlite::SQL_KIND::INSERT
    };

    sqlite::get_stmt_by_seed(conn, rng, sql_kind).unwrap_or_else(|| "SELECT 1;".to_string())
}