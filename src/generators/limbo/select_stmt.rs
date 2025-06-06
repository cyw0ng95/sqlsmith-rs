use limbo::Connection;
use crate::utils::rand_by_seed::LcgRng;

pub fn get_select_stmt_by_seed(conn: &Connection, rng: &mut LcgRng) -> Option<String> {
    let rt = tokio::runtime::Runtime::new().ok()?;

    let tables = rt.block_on(async {
        get_tables(conn).await
    }).ok()?;
    
    if tables.is_empty() {
        return None;
    }

    let table_idx = (rng.rand().unsigned_abs() as usize) % tables.len();
    let table = &tables[table_idx];
    if table.columns.is_empty() {
        return None;
    }

    let col_count = ((rng.rand().unsigned_abs() as usize) % table.columns.len()) + 1;
    let mut selected_cols = table.columns.clone();

    for i in (1..selected_cols.len()).rev() {
        let j = (rng.rand().unsigned_abs() as usize) % (i + 1);
        selected_cols.swap(i, j);
    }
    let selected_cols = &selected_cols[..col_count];

    Some(format!(
        "SELECT {} FROM {};",
        selected_cols.join(", "),
        table.name
    ))
}

// Helper struct to represent table schema
#[derive(Clone)]
struct Table {
    name: String,
    columns: Vec<String>,
}

// Helper function to get tables from Limbo connection
async fn get_tables(conn: &Connection) -> Result<Vec<Table>, Box<dyn std::error::Error>> {
    let sql: &'static str = "SELECT name FROM sqlite_schema WHERE type='table' AND name NOT LIKE 'sqlite_%';";
    let mut tables = Vec::new();

    let mut rows = conn.query(sql, ()).await.unwrap();

    while let Ok(Some(row)) = rows.next().await {

        let table_name: String = row.get_value(0).unwrap().as_text().unwrap().to_string();

        let columns = get_columns(conn, &table_name).await?;
        tables.push(Table {
            name: table_name,
            columns,
        });
    }
    
    Ok(tables)
}

// Helper function to get columns for a table
async fn get_columns(conn: &Connection, table_name: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let sql = format!("PRAGMA table_info({});", table_name);
    let mut columns = Vec::new();
    
    let mut rows = conn.query(&sql, ()).await?;
    while let Ok(Some(row)) = rows.next().await {
        let column_name: String =  row.get_value(1).unwrap().as_text().unwrap().to_string();
        columns.push(column_name);
    }
    
    Ok(columns)
}