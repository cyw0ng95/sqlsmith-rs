pub mod select_stmt;
pub mod schema;

use rusqlite::Connection;

use crate::utils::rand_by_seed::LcgRng;

pub mod insert_stmt; // 导入 insert_stmt 模块

pub enum SQL_KIND {
    SELECT,
    INSERT, // 新增 INSERT 类型
}

pub fn get_stmt_by_seed(sqlite_conn: &Connection, seeder: &mut LcgRng, kind: SQL_KIND) -> Option<String> {
    match kind {
        SQL_KIND::SELECT => select_stmt::get_select_stmt_by_seed(sqlite_conn, seeder),
        SQL_KIND::INSERT => insert_stmt::get_insert_stmt_by_seed(sqlite_conn, seeder), // 新增处理逻辑
    }
}