use limbo::Connection;
use crate::{utils::rand_by_seed::LcgRng};
pub mod schema;

pub mod select_stmt;
pub mod insert_stmt;
pub mod update_stmt;

use crate::generators::common::{gen_stmt, DriverKind, SqlKind};

pub fn get_stmt_by_seed(conn: &Connection, seeder: &mut LcgRng, kind: SqlKind) -> Option<String> {
    gen_stmt(kind, DriverKind::Limbo, conn, seeder)
}