use limbo::Connection;
use crate::{utils::rand_by_seed::LcgRng};
pub mod schema;

use crate::generators::common::{gen_stmt, DriverKind, SqlKind};
use crate::generators::common::select_stmt_common::{gen_select_stmt, TableLike};
use crate::generators::common::insert_stmt_common::{gen_insert_stmt, TableColumnLike};
use crate::generators::common::update_stmt_common::{gen_update_stmt, TableColumnLike as UpdateTableColumnLike};

impl TableLike for schema::Table {
    fn name(&self) -> &str {
        &self.name
    }
    fn columns(&self) -> Vec<String> {
        self.columns.iter().map(|(n, _)| n.clone()).collect()
    }
}

impl TableColumnLike for schema::Table {
    fn name(&self) -> &str {
        &self.name
    }
    fn columns(&self) -> Vec<(String, String)> {
        self.columns.clone()
    }
}

impl UpdateTableColumnLike for schema::Table {
    fn name(&self) -> &str {
        &self.name
    }
    fn columns(&self) -> Vec<(String, String)> {
        self.columns.clone()
    }
}

/// 辅助函数，用于获取表信息
fn get_tables_info(conn: &Connection) -> Option<Vec<schema::Table>> {
    let rt = tokio::runtime::Runtime::new().ok()?;
    let tables = rt.block_on(async { schema::get_tables(conn).await }).ok()?;
    Some(tables)
}

pub fn get_stmt_by_seed(conn: &Connection, seeder: &mut LcgRng, kind: SqlKind) -> Option<String> {
    let tables = get_tables_info(conn)?;
    match kind {
        SqlKind::Select => gen_select_stmt(&tables, seeder),
        SqlKind::Insert => gen_insert_stmt(&tables, seeder),
        SqlKind::Update => gen_update_stmt(&tables, seeder),
        _ => gen_stmt(kind, DriverKind::Limbo, conn, seeder)
    }
}

// 删除原有的 get_select_stmt_by_seed, get_insert_stmt_by_seed, get_update_stmt_by_seed 函数