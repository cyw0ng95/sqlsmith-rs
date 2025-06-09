use sqlsmith_rs_common::rand_by_seed::LcgRng;
use crate::generators::common::insert_stmt_common::TableColumnLike;

pub fn gen_upsert_stmt<T: TableColumnLike>(tables: &[T], rng: &mut LcgRng) -> Option<String> {
    if tables.is_empty() {
        return None;
    }
    let table_idx = (rng.rand().unsigned_abs() as usize) % tables.len();
    let table = &tables[table_idx];
    let columns = table.columns();
    if columns.is_empty() {
        return None;
    }

    // Generate INSERT portion
    let col_count = ((rng.rand().unsigned_abs() as usize) % columns.len()) + 1;
    let mut selected_cols = columns.iter().collect::<Vec<_>>();
    for i in (1..selected_cols.len()).rev() {
        let j = (rng.rand().unsigned_abs() as usize) % (i + 1);
        selected_cols.swap(i, j);
    }
    let selected_cols = &selected_cols[..col_count];

    // Build conflict target (using first column as example)
    let conflict_target = selected_cols.first().map(|(name, _)| name).unwrap();

    // Generate UPDATE portion
    let update_clause: Vec<String> = selected_cols
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
        "INSERT INTO {} ({}) VALUES ({}) ON CONFLICT({}) DO UPDATE SET {};",
        table.name(),
        selected_cols.iter().map(|(n, _)| n.as_str()).collect::<Vec<_>>().join(", "),
        selected_cols.iter().map(|(_, t)| match t.to_uppercase().as_str() {
            "INTEGER" => (rng.rand().abs() % 1000).to_string(),
            "REAL" => format!("{}", (rng.rand().abs() as f64) / 100.0),
            "TEXT" => format!("'val{}'", rng.rand().abs() % 1000),
            "BLOB" => "'blob'".to_string(),
            _ => "NULL".to_string(),
        }).collect::<Vec<_>>().join(", "),
        conflict_target,
        update_clause.join(", ")
    ))
}