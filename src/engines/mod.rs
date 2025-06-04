use rusqlite::Connection;
use crate::utils::rand_by_seed::LcgRng;
use crate::generators::sqlite;
use crate::drivers::DatabaseDriver;
use log::info;

pub struct Engine<'a> {
    pub rng: LcgRng,
    pub driver: &'a dyn DatabaseDriver<Connection = Connection>,
}

impl<'a> Engine<'a> {
    pub fn new(seed: u64, driver: &'a dyn DatabaseDriver<Connection = Connection>) -> Self {
        Self {
            rng: LcgRng::new(seed),
            driver,
        }
    }

    pub fn next_sql(&mut self, conn: &Connection) -> Option<String> {
        let is_select = self.rng.rand().unsigned_abs() % 2 == 0;
        let sql_kind = if is_select {
            sqlite::SQL_KIND::SELECT
        } else {
            sqlite::SQL_KIND::INSERT
        };
        sqlite::get_stmt_by_seed(conn, &mut self.rng, sql_kind)
    }

    pub fn exec(&self, conn: &mut Connection, sql: &str) -> anyhow::Result<usize> {
        if sql.trim_start().to_uppercase().starts_with("SELECT") {
            // 处理 SELECT 语句
            let mut stmt = conn.prepare(sql)?;
            let mut rows = stmt.query([])?;
            while let Some(_) = rows.next()? {}
            Ok(0) // SELECT 语句通常返回受影响行数为 0
        } else {
            // 处理其他语句
            self.driver.exec(conn, sql)
        }
    }

    pub fn run(&mut self, conn: &mut Connection, count: usize) {
        let mut i = 0;
        while i < count {
            let sql = self.next_sql(conn)
                .unwrap_or_else(|| "SELECT 1;".to_string());
            info!("Generated SQL: {}", sql);

            let result = self.exec(conn, &sql);
            match result {
                Ok(_) => {}
                Err(e) => {
                    i += 1;
                    info!("Error executing SQL with ret: [{:?}]", e);
                    continue;
                }
            }
            i += 1;
        }
    }
}