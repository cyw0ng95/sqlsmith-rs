pub mod select_stmt;
pub mod schema;

use rusqlite::Connection;

use crate::utils::rand_by_seed::LcgRng;
use crate::generators::common::{gen_stmt, DriverKind, SqlKind};

pub mod insert_stmt; // 导入 insert_stmt 模块
pub mod update_stmt; // 确保正确导入模块
pub mod pragma_stmt; // 新增 pragma_stmt 模块

pub fn get_stmt_by_seed(sqlite_conn: &Connection, seeder: &mut LcgRng, kind: SqlKind) -> Option<String> {
    gen_stmt(kind, DriverKind::Sqlite, sqlite_conn, seeder)
}