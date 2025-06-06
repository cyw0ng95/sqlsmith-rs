use crate::drivers::DatabaseDriver;
use crate::engines::Engine;
use crate::{drivers::DRIVER_KIND, utils::rand_by_seed::LcgRng}; // 请替换 `some_module` 为实际定义 `Engine` 的模块路径

pub mod sqlite;
pub mod limbo;

#[derive(Debug, Clone, Copy)]
pub enum SQL_KIND {
    SELECT,
    INSERT,
    UPDATE,
    VACUUM,
}

pub async fn get_stmt_by_seed(
    conn: &mut dyn std::any::Any,
    rng: &mut LcgRng,
    kind: SQL_KIND,
    driver_kind: DRIVER_KIND,
) -> Option<String> {
    match driver_kind {
        DRIVER_KIND::SQLITE_IN_MEM => {
            // 尝试将 conn downcast 到 rusqlite::Connection
            if let Some(sqlite_conn) = conn.downcast_mut::<rusqlite::Connection>() {
                sqlite::get_stmt_by_seed(sqlite_conn, rng, kind)
            } else {
                None
            }
        }
        DRIVER_KIND::LIMBO => {
            if let Some(limbo_conn) = conn.downcast_mut::<::limbo::Connection>() {
                limbo::get_stmt_by_seed(limbo_conn, rng, kind).await
            } else {
                None
            }
        }
        _ => None,
    }
}
