mod sqlite;

use rusqlite::Connection;

/// 生成 SQL 语句，需传入数据库连接和种子
pub fn generate(conn: &Connection, seed: i64) -> String {
    sqlite::select_stmt::get_stmt_by_seed(conn, seed)
        .unwrap_or_else(|| "SELECT 1;".to_string()) // 默认返回 SELECT 1;
}