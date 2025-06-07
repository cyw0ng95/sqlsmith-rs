use limbo::Connection;
use crate::utils::rand_by_seed::LcgRng;
use crate::generators::common::select_stmt_common::{gen_select_stmt, TableLike};

impl TableLike for super::schema::Table {
    fn name(&self) -> &str {
        &self.name
    }
    fn columns(&self) -> Vec<String> {
        self.columns.iter().map(|(n, _)| n.clone()).collect()
    }
}

pub fn get_select_stmt_by_seed(conn: &Connection, rng: &mut LcgRng) -> Option<String> {
    let rt = tokio::runtime::Runtime::new().ok()?;
    let tables = rt.block_on(async { super::schema::get_tables(conn).await }).ok()?;
    gen_select_stmt(&tables, rng)
}