use sqlsmith_rs_drivers::DatabaseDriver;
use crate::engines::Engine;
use crate::generators::common::SqlKind;
use sqlsmith_rs_drivers::DRIVER_KIND;
use sqlsmith_rs_common::rand_by_seed::LcgRng;

pub mod sqlite;
pub mod limbo;
pub mod common;

pub fn get_stmt_by_seed(
    seeder: &mut LcgRng,
    kind: SqlKind,
    mut engine: Box<dyn Engine>,
) -> Option<String> {
    match engine.get_driver_kind() {
        DRIVER_KIND::SQLITE_IN_MEM => {
            let driver_box = engine.get_sqlite_driver_box();
            let conn = match driver_box {
                Some(box_instance) => box_instance.get_connection_mut(),
                None => {
                    log::error!("Failed to get SQLite driver connection");
                    return None;
                }
            };
            sqlite::get_stmt_by_seed(conn, seeder, kind)
        }
        DRIVER_KIND::LIMBO_IN_MEM => {
            let driver_box = engine.get_limbo_driver_box();
            let conn = match driver_box {
                Some(box_instance) => box_instance.get_connection_mut(),
                None => {
                    log::error!("Failed to get Limbo driver connection");
                    return None;
                }
            };
            limbo::get_stmt_by_seed(conn, seeder, kind)
        }
    }
}
