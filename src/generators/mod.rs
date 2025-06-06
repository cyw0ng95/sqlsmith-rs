use crate::drivers::DatabaseDriver;
use crate::engines::Engine;
use crate::{drivers::DRIVER_KIND, generators::sqlite::SQL_KIND, utils::rand_by_seed::LcgRng};

pub mod sqlite;
pub mod limbo;

pub fn get_stmt_by_seed(
    seeder: &mut LcgRng,
    kind: SQL_KIND,
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
        DRIVER_KIND::LIMBO => {
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
