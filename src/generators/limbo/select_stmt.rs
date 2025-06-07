use limbo::Connection;
use crate::utils::rand_by_seed::LcgRng;
use super::schema::get_tables; // 加入这一行

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
    let mut selected_cols: Vec<_> = table.columns.iter().map(|(name, _)| name.clone()).collect();

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