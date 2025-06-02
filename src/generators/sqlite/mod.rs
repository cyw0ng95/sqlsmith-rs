pub mod select_stmt;
pub mod schema;

use rusqlite::Connection;

pub enum SQL_KIND {
    SELECT,
}

pub fn get_stmt_by_seed(sqlite_conn: &Connection, seed: i64, kind: SQL_KIND) -> Option<String> {
    match kind {
        SQL_KIND::SELECT => select_stmt::get_stmt_by_seed(sqlite_conn, seed),
    }
}