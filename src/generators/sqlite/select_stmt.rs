use rusqlite::Connection;
use crate::generators::sqlite::schema;
use crate::utils::rand_by_seed::LcgRng;

pub fn get_select_stmt_by_seed(sqlite_conn: &Connection, rng: &mut LcgRng) -> Option<String> {
    let tables = match schema::get(sqlite_conn) {
        Ok(t) if !t.is_empty() => t,
        _ => return None,
    };

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