use limbo::Connection;
use sqlsmith_rs_common::rand_by_seed::LcgRng;
pub mod schema;

use crate::generators::common::insert_stmt_common::{TableColumnLike, gen_insert_stmt};
use crate::generators::common::select_stmt_common::{TableLike, gen_select_stmt};
use crate::generators::common::update_stmt_common::{
    TableColumnLike as UpdateTableColumnLike, gen_update_stmt,
};
use crate::generators::common::{DriverKind, SqlKind, gen_stmt};

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
        SqlKind::Delete => {
            crate::generators::common::delete_stmt_common::gen_delete_stmt(&tables, seeder)
        }
        SqlKind::Vacuum => crate::generators::common::vacuum_stmt_common::gen_vacuum_stmt(),
        SqlKind::Pragma => {
            // Limbo 目前对 Pragma 无处理，可保持 None 或后续添加实现
            None
        }
        _ => gen_stmt(kind, DriverKind::Limbo, conn, seeder),
    }
}

// 删除原有的 get_select_stmt_by_seed, get_insert_stmt_by_seed, get_update_stmt_by_seed 函数
