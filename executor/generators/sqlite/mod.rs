use rusqlite::Connection;
use sqlsmith_rs_common::rand_by_seed::LcgRng;
use crate::generators::common::{gen_stmt, DriverKind, SqlKind};
pub mod schema;

// 集成 select_stmt.rs
use crate::generators::common::select_stmt_common::{gen_select_stmt, TableLike};
impl TableLike for schema::TableInfo {
    fn name(&self) -> &str {
        &self.name
    }
    fn columns(&self) -> Vec<String> {
        self.columns.clone()
    }
}

// 集成 insert_stmt.rs
use crate::generators::common::insert_stmt_common::{TableColumnLike, gen_insert_stmt};
// 修改结构体，直接拥有 name 的所有权
struct TableWithColumns {
    name: String,
    columns: Vec<(String, String)>,
}
impl TableColumnLike for TableWithColumns {
    fn name(&self) -> &str {
        &self.name
    }
    fn columns(&self) -> Vec<(String, String)> {
        self.columns.clone()
    }
}

// 集成 update_stmt.rs
use crate::generators::common::update_stmt_common::{gen_update_stmt, TableColumnLike as UpdateTableColumnLike};
// 修改结构体，直接拥有 name 的所有权
struct UpdateTableWithColumns {
    name: String,
    columns: Vec<(String, String)>,
}
impl UpdateTableColumnLike for UpdateTableWithColumns {
    fn name(&self) -> &str {
        &self.name
    }
    fn columns(&self) -> Vec<(String, String)> {
        self.columns.clone()
    }
}

// Add new struct implementing TriggerTableLike trait from common module
struct TriggerTableWithColumns {
    name: String,
    columns: Vec<(String, String)>,
    has_primary_key: bool,
}

impl crate::generators::common::create_trigger_stmt_common::TriggerTableLike for TriggerTableWithColumns {
    fn name(&self) -> &str {
        &self.name
    }
    fn columns(&self) -> Vec<(String, String)> {
        self.columns.clone()
    }
    fn has_primary_key(&self) -> bool {
        self.has_primary_key
    }
}

pub fn get_stmt_by_seed(sqlite_conn: &Connection, seeder: &mut LcgRng, kind: SqlKind) -> Option<String> {
    match kind {
        SqlKind::Select => {
            let tables = match schema::get(sqlite_conn) {
                Ok(t) if !t.is_empty() => t,
                _ => return None,
            };
            gen_select_stmt(&tables, seeder)
        },
        SqlKind::Insert => {
            let tables_with_columns = schema::get_tables_with_columns(sqlite_conn);
            let mut wrapped_tables = Vec::new();
            for (name, columns) in tables_with_columns {
                wrapped_tables.push(TableWithColumns {
                    name,
                    columns,
                });
            }
            gen_insert_stmt(&wrapped_tables, seeder)
        },
        SqlKind::Update => {
            let tables_with_columns = schema::get_tables_with_columns(sqlite_conn);
            let mut wrapped_tables = Vec::new();
            for (name, columns) in tables_with_columns {
                wrapped_tables.push(UpdateTableWithColumns {
                    name,
                    columns,
                });
            }
            gen_update_stmt(&wrapped_tables, seeder)
        },
        SqlKind::Delete => {
            let tables_with_columns = schema::get_tables_with_columns(sqlite_conn);
            let mut wrapped_tables = Vec::new();
            for (name, columns) in tables_with_columns {
                wrapped_tables.push(TableWithColumns {
                    name,
                    columns,
                });
            }
            crate::generators::common::delete_stmt_common::gen_delete_stmt(&wrapped_tables, seeder)
        },
        SqlKind::Vacuum => {
            crate::generators::common::vacuum_stmt_common::gen_vacuum_stmt()
        },
        SqlKind::Pragma => {
            crate::generators::common::pragma_stmt_common::get_pragma_stmt_by_seed(sqlite_conn, seeder)
        },
        SqlKind::CreateTrigger => {
            let tables_with_columns = schema::get_tables_with_columns(sqlite_conn);
            let mut wrapped_tables = Vec::new();
            for (name, columns) in tables_with_columns {
                // Assume we check primary key existence via schema (simplified example)
                let has_primary_key = columns.iter().any(|(col, _)| col == "id"); // Adjust based on actual schema logic
                wrapped_tables.push(TriggerTableWithColumns {
                    name,
                    columns,
                    has_primary_key,
                });
            }
            crate::generators::common::create_trigger_stmt_common::gen_create_trigger_stmt(&wrapped_tables, seeder)
        },
        _ => gen_stmt(kind, DriverKind::Sqlite, sqlite_conn, seeder)
    }
}

// 移除原有的 get_select_stmt_by_seed, get_insert_stmt_by_seed, get_update_stmt_by_seed 函数