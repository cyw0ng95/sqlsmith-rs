use rusqlite::Connection;
use crate::generators::sqlite::schema;
use crate::utils::rand_by_seed::LcgRng;
use crate::generators::common::insert_stmt_common::TableColumnLike;
use crate::generators::common::insert_stmt_common::gen_insert_stmt;

struct TableWithColumns<'a> {
    name: &'a str,
    columns: Vec<(String, String)>,
}

impl<'a> TableColumnLike for TableWithColumns<'a> {
    fn name(&self) -> &str {
        self.name
    }
    fn columns(&self) -> Vec<(String, String)> {
        self.columns.clone()
    }
}

pub fn get_insert_stmt_by_seed(sqlite_conn: &Connection, rng: &mut LcgRng) -> Option<String> {
    let tables = match schema::get(sqlite_conn) {
        Ok(t) if !t.is_empty() => t,
        _ => return None,
    };
    let mut tables_with_columns = Vec::new();
    for table in &tables {
        let mut stmt = match sqlite_conn.prepare(&format!("PRAGMA table_info({})", table.name)) {
            Ok(stmt) => stmt,
            Err(_) => continue,
        };
        let columns_info = match stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        }) {
            Ok(rows) => {
                let mut info = Vec::new();
                for row in rows {
                    if let Ok((name, col_type)) = row {
                        info.push((name, col_type));
                    }
                }
                info
            }
            Err(_) => continue,
        };
        tables_with_columns.push(TableWithColumns {
            name: &table.name,
            columns: columns_info,
        });
    }
    gen_insert_stmt(&tables_with_columns, rng)
}