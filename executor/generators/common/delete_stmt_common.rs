use crate::generators::common::insert_stmt_common::TableColumnLike;
use sqlsmith_rs_common::rand_by_seed::LcgRng;

pub fn gen_delete_stmt<T: TableColumnLike>(tables: &[T], rng: &mut LcgRng) -> Option<String> {
    if tables.is_empty() {
        return None;
    }
    let table_idx = (rng.rand().unsigned_abs() as usize) % tables.len();
    let table = &tables[table_idx];
    let columns = table.columns();

    if columns.is_empty() {
        return None;
    }

    let where_clause = columns
        .iter()
        .take(1) // Use first column for WHERE condition
        .map(|(name, _)| {
            format!(
                "{} = {}",
                name,
                match rng.rand().abs() % 4 {
                    0 => (rng.rand().abs() % 1000).to_string(),
                    1 => format!("'val{}'", rng.rand().abs() % 1000),
                    2 => "NULL".to_string(),
                    _ => "1".to_string(),
                }
            )
        })
        .collect::<Vec<_>>();

    Some(format!(
        "DELETE FROM {} WHERE {};",
        table.name(),
        where_clause.join(" AND ")
    ))
}
