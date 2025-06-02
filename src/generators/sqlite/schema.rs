use rusqlite::{Connection, Result};

#[derive(Debug, Clone)]
pub struct TableInfo {
    pub name: String,
    pub columns: Vec<String>,
}

pub fn get(sqlite_conn: &Connection) -> Result<Vec<TableInfo>> {
    let mut tables = Vec::new();
    let mut stmt = sqlite_conn.prepare(
        "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%';"
    )?;
    let table_names = stmt.query_map([], |row| row.get::<_, String>(0))?;

    for table_name in table_names {
        let table_name = table_name?;
        let mut col_stmt = sqlite_conn.prepare(&format!("PRAGMA table_info('{}')", table_name))?;
        let columns = col_stmt
            .query_map([], |row| row.get::<_, String>(1))?
            .collect::<Result<Vec<_>, _>>()?;
        tables.push(TableInfo {
            name: table_name,
            columns,
        });
    }
    Ok(tables)
}