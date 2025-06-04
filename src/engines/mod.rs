use rusqlite::Connection;
use crate::utils::rand_by_seed::LcgRng;
use crate::generators::sqlite;
use crate::drivers::DatabaseDriver;
use log::info;

pub struct Engine<'a> {
    pub rng: LcgRng,
    pub driver: &'a dyn DatabaseDriver<Connection = Connection>,
}

use serde_json::Value;
use std::fs::File;
use std::io::Read;

impl<'a> Engine<'a> {
    pub fn new(seed: u64, driver: &'a dyn DatabaseDriver<Connection = Connection>) -> Self {
        Self {
            rng: LcgRng::new(seed),
            driver,
        }
    }

    pub fn next_sql(&mut self, conn: &Connection) -> Option<String> {
        let mut file = File::open("profile.json").expect("Failed to open profile.json");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read profile.json");
        let json: Value = serde_json::from_str(&contents).expect("Failed to parse profile.json");

        let select_prob = json["stmt_prob"]["SELECT"].as_u64().unwrap_or(100);
        let insert_prob = json["stmt_prob"]["INSERT"].as_u64().unwrap_or(50);
        let update_prob = json["stmt_prob"]["UPDATE_STMT"].as_u64().unwrap_or(30);
        let total = select_prob + insert_prob + update_prob;

        let random_num = self.rng.rand().unsigned_abs() as u64 % total;
        let sql_kind = if random_num < select_prob {
            sqlite::SQL_KIND::SELECT
        } else if random_num < select_prob + insert_prob {
            sqlite::SQL_KIND::INSERT
        } else {
            sqlite::SQL_KIND::UPDATE
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