pub mod sqlite;

use rusqlite::Connection;
use crate::utils::rand_by_seed::LcgRng;

pub fn generate(conn: &Connection, rng: &mut LcgRng) -> String {
    let is_select = rng.rand().unsigned_abs() % 2 == 0;
    let sql_kind = if is_select {
        sqlite::SQL_KIND::SELECT
    } else {
        sqlite::SQL_KIND::INSERT
    };
    sqlite::get_stmt_by_seed(conn, rng, sql_kind).unwrap_or_else(|| "SELECT 1;".to_string())
}