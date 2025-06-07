use limbo::Connection;
use crate::utils::rand_by_seed::LcgRng;
use crate::generators::common::insert_stmt_common::{gen_insert_stmt, TableColumnLike};

impl TableColumnLike for super::schema::Table {
    fn name(&self) -> &str {
        &self.name
    }
    fn columns(&self) -> Vec<(String, String)> {
        self.columns.clone()
    }
}

pub fn get_insert_stmt_by_seed(conn: &Connection, rng: &mut LcgRng) -> Option<String> {
    let rt = tokio::runtime::Runtime::new().ok()?;
    let tables = rt.block_on(async { super::schema::get_tables(conn).await }).ok()?;
    gen_insert_stmt(&tables, rng)
}