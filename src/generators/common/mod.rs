// 仅用于公开 common 子模块
pub mod select_stmt_common;
pub mod update_stmt_common;
pub mod insert_stmt_common;
pub mod vacuum_stmt_common;
pub mod pragma_stmt_common;

// 通用 SQL 语句类型定义，供 limbo 和 sqlite 共享
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SqlKind {
    Select,
    Insert,
    Update,
    Delete,
    Vacuum,
    Pragma, // 某些数据库特有，可选
    // ...如有需要可扩展
}

use crate::utils::rand_by_seed::LcgRng;

pub enum DriverKind {
    Sqlite,
    Limbo,
}

pub fn gen_stmt(
    sql_kind: SqlKind,
    driver_kind: DriverKind,
    conn: &dyn std::any::Any,
    rng: &mut LcgRng,
) -> Option<String> {
    match (sql_kind, driver_kind) {
        (SqlKind::Select, DriverKind::Sqlite) => {
            if let Some(sqlite_conn) = conn.downcast_ref::<rusqlite::Connection>() {
                crate::generators::sqlite::get_stmt_by_seed(sqlite_conn, rng, SqlKind::Select)
            } else {
                None
            }
        }
        (SqlKind::Select, DriverKind::Limbo) => {
            if let Some(limbo_conn) = conn.downcast_ref::<limbo::Connection>() {
                crate::generators::limbo::get_stmt_by_seed(limbo_conn, rng, SqlKind::Select)
            } else {
                None
            }
        }
        (SqlKind::Insert, DriverKind::Sqlite) => {
            if let Some(sqlite_conn) = conn.downcast_ref::<rusqlite::Connection>() {
                crate::generators::sqlite::get_stmt_by_seed(sqlite_conn, rng, SqlKind::Insert)
            } else {
                None
            }
        }
        (SqlKind::Insert, DriverKind::Limbo) => {
            if let Some(limbo_conn) = conn.downcast_ref::<limbo::Connection>() {
                crate::generators::limbo::get_stmt_by_seed(limbo_conn, rng, SqlKind::Insert)
            } else {
                None
            }
        }
        (SqlKind::Update, DriverKind::Sqlite) => {
            if let Some(sqlite_conn) = conn.downcast_ref::<rusqlite::Connection>() {
                crate::generators::sqlite::get_stmt_by_seed(sqlite_conn, rng, SqlKind::Update)
            } else {
                None
            }
        }
        (SqlKind::Update, DriverKind::Limbo) => {
            if let Some(limbo_conn) = conn.downcast_ref::<limbo::Connection>() {
                crate::generators::limbo::get_stmt_by_seed(limbo_conn, rng, SqlKind::Update)
            } else {
                None
            }
        }
        (SqlKind::Vacuum, _) => {
            crate::generators::common::vacuum_stmt_common::gen_vacuum_stmt()
        }
        (SqlKind::Pragma, DriverKind::Sqlite) => {
            if let Some(sqlite_conn) = conn.downcast_ref::<rusqlite::Connection>() {
                crate::generators::common::pragma_stmt_common::get_pragma_stmt_by_seed(sqlite_conn, rng)
            } else {
                None
            }
        }
        // Limbo 不支持 PRAGMA
        (SqlKind::Pragma, DriverKind::Limbo) => None,
        _ => None,
    }
}
