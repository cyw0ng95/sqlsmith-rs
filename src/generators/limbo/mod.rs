use limbo::Connection;
use crate::{utils::rand_by_seed::LcgRng};
pub mod schema;

pub mod select_stmt;
pub mod insert_stmt;
pub mod update_stmt;
pub mod vacuum_stmt;

pub fn get_stmt_by_seed(conn: &Connection, seeder: &mut LcgRng, kind: super::SQL_KIND) -> Option<String> {
    match kind {
        super::SQL_KIND::SELECT => select_stmt::get_select_stmt_by_seed(conn, seeder),
        super::SQL_KIND::INSERT => insert_stmt::get_insert_stmt_by_seed(conn, seeder),
        super::SQL_KIND::UPDATE => update_stmt::get_update_stmt_by_seed(conn, seeder),
        super::SQL_KIND::VACUUM => vacuum_stmt::get_vacuum_stmt_by_seed(conn, seeder),
    }
}