pub mod sqlite;

use rusqlite::Connection;
use crate::utils::rand_by_seed::LcgRng;

pub fn generate(conn: &Connection, rng: &mut LcgRng) -> String {
    sqlite::select_stmt::get_stmt_by_seed(conn, rng)
        .unwrap_or_else(|| "SELECT 1;".to_string()) // 默认返回 SELECT 1;
}