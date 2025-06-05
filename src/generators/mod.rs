use limbo::Connection;

use crate::drivers::DatabaseDriver;
use crate::engines::Engine;
use crate::{drivers::DRIVER_KIND, generators::sqlite::SQL_KIND, utils::rand_by_seed::LcgRng}; // 请替换 `some_module` 为实际定义 `Engine` 的模块路径

pub mod sqlite;

// 假设 `Engine` 应该是一个 trait，当前代码可能将其错误当作结构体使用，这里保持使用 `Box<dyn Engine>` 表示是一个动态 trait 对象
pub fn get_stmt_by_seed(
    seeder: &mut LcgRng,
    kind: SQL_KIND,
    mut engine: Box<dyn Engine>, // 改为可变 engine（因为需要调用 get_sqlite_driver_box 的可变方法）
) -> Option<String> {
    match engine.get_driver_kind() {
        DRIVER_KIND::SQLITE_IN_MEM => {
            let driver_box = engine.get_sqlite_driver_box(); // 现在获取的是可变引用
            let conn = match driver_box {
                Some(box_instance) => box_instance.get_connection_mut(), // 可变引用允许调用 get_connection_mut
                None => {
                    log::error!("Failed to get SQLite driver connection");
                    return None;
                }
            };
            sqlite::get_stmt_by_seed(conn, seeder, kind)
        }
        DRIVER_KIND::LIMBO => {
            panic!("awaiting impl")
        }
    }
}
