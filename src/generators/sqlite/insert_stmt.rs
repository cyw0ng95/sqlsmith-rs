use rusqlite::Connection;
use crate::generators::sqlite::schema;
use crate::utils::rand_by_seed::LcgRng;

pub fn get_insert_stmt_by_seed(sqlite_conn: &Connection, rng: &mut LcgRng) -> Option<String> {
    let tables = match schema::get(sqlite_conn) {
        Ok(t) if !t.is_empty() => t,
        _ => return None,
    };

    let table_idx = (rng.rand().unsigned_abs() as usize) % tables.len();
    let table = &tables[table_idx];

    // 使用 PRAGMA 语句获取表的列信息
    let mut stmt = match sqlite_conn.prepare(&format!("PRAGMA table_info({})", table.name)) {
        Ok(stmt) => stmt,
        Err(_) => return None,
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
        Err(_) => return None,
    };

    let mut col_names = Vec::new();
    let mut values = Vec::new();

    for (name, col_type) in columns_info {
        col_names.push(name);
        let random_value = match col_type.as_str() {
            "INTEGER" => format!("{}", rng.rand().unsigned_abs() % 1000),
            "TEXT" => {
                let mut text = String::new();
                for _ in 0..10 {
                    let char_code = (rng.rand().unsigned_abs() % 26) + 97;
                    text.push(char_code as u8 as char);
                }
                format!("'{}'", text)
            }
            "REAL" => {
                let random_num = (rng.rand().unsigned_abs() % 10000) as f64 / 100.0;
                format!("{:.2}", random_num)
            }
            _ => "'default_value'".to_string(),
        };
        values.push(random_value);
    }

    Some(format!(
        "INSERT INTO `{}` ({}) VALUES ({});",
        table.name,
        col_names.join(", "),
        values.join(", ")
    ))
}