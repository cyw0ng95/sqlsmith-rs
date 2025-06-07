use limbo::Connection;
use crate::utils::rand_by_seed::LcgRng;
use super::schema::{get_tables, Table};

pub fn get_update_stmt_by_seed(conn: &Connection, rng: &mut LcgRng) -> Option<String> {
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

    // 随机选择要更新的列数量
    let col_count = ((rng.rand().unsigned_abs() as usize) % table.columns.len()) + 1;
    let selected_cols: Vec<(String, String)> = {
        let mut cols = table.columns.clone();
        for i in (1..cols.len()).rev() {
            let j = (rng.rand().unsigned_abs() as usize) % (i + 1);
            cols.swap(i, j);
        }
        cols.into_iter().take(col_count).collect()
    };

    // 生成 SET 子句
    let set_clause: Vec<String> = selected_cols
        .iter()
        .map(|(name, ty)| {
            let value = match ty.to_uppercase().as_str() {
                "INTEGER" => (rng.rand().abs() % 1000).to_string(),
                "REAL" => format!("{}", (rng.rand().abs() as f64) / 100.0),
                "TEXT" => format!("'val{}'", rng.rand().abs() % 1000),
                "BLOB" => "'blob'".to_string(),
                _ => "NULL".to_string(),
            };
            format!("{} = {}", name, value)
        })
        .collect();

    Some(format!(
        "UPDATE {} SET {} WHERE 1=1;",
        table.name,
        set_clause.join(", ")
    ))
}