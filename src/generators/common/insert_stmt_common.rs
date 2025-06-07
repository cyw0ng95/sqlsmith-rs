// 通用 INSERT 语句生成逻辑，供 limbo/sqlite 共享
use crate::utils::rand_by_seed::LcgRng;

pub trait TableColumnLike {
    fn name(&self) -> &str;
    fn columns(&self) -> Vec<(String, String)>; // (name, type)
}

pub fn gen_insert_stmt<T: TableColumnLike>(tables: &[T], rng: &mut LcgRng) -> Option<String> {
    if tables.is_empty() {
        return None;
    }
    let table_idx = (rng.rand().unsigned_abs() as usize) % tables.len();
    let table = &tables[table_idx];
    let columns = table.columns();
    if columns.is_empty() {
        return None;
    }
    // 随机选择要插入的列数量
    let col_count = ((rng.rand().unsigned_abs() as usize) % columns.len()) + 1;
    let mut selected_cols = columns.clone();
    for i in (1..selected_cols.len()).rev() {
        let j = (rng.rand().unsigned_abs() as usize) % (i + 1);
        selected_cols.swap(i, j);
    }
    let selected_cols = &selected_cols[..col_count];
    let col_names: Vec<&str> = selected_cols.iter().map(|(name, _)| name.as_str()).collect();
    let values: Vec<String> = selected_cols
        .iter()
        .map(|(_, ty)| match ty.to_uppercase().as_str() {
            "INTEGER" => (rng.rand().abs() % 1000).to_string(),
            "REAL" => format!("{}", (rng.rand().abs() as f64) / 100.0),
            "TEXT" => format!("'val{}'", rng.rand().abs() % 1000),
            "BLOB" => "'blob'".to_string(),
            _ => "NULL".to_string(),
        })
        .collect();
    Some(format!(
        "INSERT INTO {} ({}) VALUES ({});",
        table.name(),
        col_names.join(", "),
        values.join(", ")
    ))
}
