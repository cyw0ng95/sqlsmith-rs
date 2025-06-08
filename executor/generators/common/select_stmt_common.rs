// 通用 SELECT 语句生成逻辑，供 limbo/sqlite 共享
// TableInfo: 需实现 name: &str, columns: &[String] trait
use crate::utils::rand_by_seed::LcgRng;

pub trait TableLike {
    fn name(&self) -> &str;
    fn columns(&self) -> Vec<String>;
}

pub fn gen_select_stmt<T: TableLike>(tables: &[T], rng: &mut LcgRng) -> Option<String> {
    if tables.is_empty() {
        return None;
    }
    let table_idx = (rng.rand().unsigned_abs() as usize) % tables.len();
    let table = &tables[table_idx];
    let columns = table.columns();
    if columns.is_empty() {
        return None;
    }
    let col_count = ((rng.rand().unsigned_abs() as usize) % columns.len()) + 1;
    let mut selected_cols = columns;
    for i in (1..selected_cols.len()).rev() {
        let j = (rng.rand().unsigned_abs() as usize) % (i + 1);
        selected_cols.swap(i, j);
    }
    let selected_cols = &selected_cols[..col_count];
    Some(format!(
        "SELECT {} FROM {};",
        selected_cols.join(", "),
        table.name()
    ))
}
