use rusqlite::Connection;
use crate::generators::common::select_stmt_common::{gen_select_stmt, TableLike};
use crate::generators::sqlite::schema;
use crate::utils::rand_by_seed::LcgRng;

impl TableLike for schema::TableInfo {
    fn name(&self) -> &str {
        &self.name
    }
    fn columns(&self) -> Vec<String> {
        self.columns.clone()
    }
}

pub fn get_select_stmt_by_seed(sqlite_conn: &Connection, rng: &mut LcgRng) -> Option<String> {
    let tables = match schema::get(sqlite_conn) {
        Ok(t) if !t.is_empty() => t,
        _ => return None,
    };
    gen_select_stmt(&tables, rng)
}