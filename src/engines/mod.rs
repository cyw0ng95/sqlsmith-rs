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
use std::time::{Instant, Duration};

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
        let update_prob = json["stmt_prob"]["UPDATE"].as_u64().unwrap_or(50);
        let vacuum_prob = json["stmt_prob"]["VACUUM"].as_u64().unwrap_or(20); // 新增 VACUUM 概率
        let total = select_prob + insert_prob + update_prob + vacuum_prob;

        let random_num = self.rng.rand().unsigned_abs() as u64 % total;
        let sql_kind = if random_num < select_prob {
            sqlite::SQL_KIND::SELECT
        } else if random_num < select_prob + insert_prob {
            sqlite::SQL_KIND::INSERT
        } else if random_num < select_prob + insert_prob + update_prob {
            sqlite::SQL_KIND::UPDATE
        } else {
            sqlite::SQL_KIND::VACUUM // 新增 VACUUM 处理逻辑
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
        let start_time = Instant::now();
        let mut last_print_time = start_time;
        let mut executed_count = 0;
        let mut success_count = 0;
        let mut failed_count = 0;

        let mut i = 0;
        while i < count {
            let sql = self.next_sql(conn)
                .unwrap_or_else(|| "SELECT 1;".to_string());
            // info!("Generated SQL: {}", sql);

            let exec_start = Instant::now();
            let result = self.exec(conn, &sql);
            let exec_duration = exec_start.elapsed();

            match result {
                Ok(_) => {
                    // info!("SQL executed successfully in {:?}", exec_duration);
                    success_count += 1;
                }
                Err(e) => {
                    i += 1;
                    failed_count += 1;
                    // info!("Error executing SQL with ret: [{:?}] in {:?}", e, exec_duration);
                    continue;
                }
            }
            i += 1;
            executed_count += 1;

            let current_time = Instant::now();
            if current_time.duration_since(last_print_time) >= Duration::from_secs(1) {
                let elapsed_seconds = current_time.duration_since(start_time).as_secs_f64();
                let speed = executed_count as f64 / elapsed_seconds;
                info!("Execution speed: {:.2} SQL statements per second, Sum: Success/Failed: {}/{}", speed, success_count, failed_count);
                last_print_time = current_time;
            }
        }
    }
}