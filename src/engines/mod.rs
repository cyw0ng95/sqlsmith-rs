use rusqlite::Connection;
use crate::drivers::{DatabaseDriver, DRIVER_KIND, new_conn};
use crate::utils::rand_by_seed::LcgRng;
use crate::profile::Profile;

pub struct Engine<'a> {
    pub rng: LcgRng,
    pub driver: Box<dyn DatabaseDriver<Connection = Connection> + 'a>,
    pub conn: Box<Connection>,
    pub run_count: usize,
    pub stmt_prob: Option<crate::profile::StmtProb>,
    pub debug: Option<crate::profile::DebugOptions>,  // 新增 debug 字段
}

impl<'a> Engine<'a> {
    pub fn with_driver_kind(seed: u64, kind: DRIVER_KIND, run_count: usize, profile: &Profile) -> anyhow::Result<Self> {
        let (driver, conn) = new_conn(kind)?;
        Ok(Self {
            rng: LcgRng::new(seed),
            driver,
            conn,
            run_count,
            stmt_prob: profile.stmt_prob.clone(),
            debug: profile.debug.clone(),  // 添加 debug 选项的支持
        })
    }

    pub fn run(&mut self) {
        let mut i = 0;
        while i < self.run_count {
            let sql = self.generate_sql();
            
            // 根据 SQL 类型选择执行方法
            let result = if sql.trim_start().to_uppercase().starts_with("SELECT") {
                self.driver.query(&mut self.conn, &sql)
            } else {
                self.driver.exec(&mut self.conn, &sql)
            };

            match result {
                Ok(affected) => {
                    if let Some(debug) = &self.debug {
                        if debug.show_success_sql {
                            log::info!("SQL executed successfully: {} (affected: {})", sql, affected);
                        }
                    }
                }
                Err(e) => {
                    if let Some(debug) = &self.debug {
                        if debug.show_failed_sql {
                            log::info!("Error executing SQL: {} with ret: [{:?}]", sql, e);
                        }
                    }
                }
            }
            i += 1;
        }
    }

    fn generate_sql(&mut self) -> String {
        // 根据 stmt_prob 选择 SQL 类型
        if let Some(prob) = &self.stmt_prob {
            let total = prob.SELECT + prob.INSERT + prob.UPDATE + prob.VACUUM;
            if total == 0 {
                return "SELECT 1;".to_string();
            }
            
            // 将 rand() 的 i64 结果转为非负数，然后按比例计算
            let r = (self.rng.rand().abs() as u64) % total;
            
            if r < prob.SELECT {
                return crate::generators::sqlite::get_stmt_by_seed(
                    &self.conn,
                    &mut self.rng,
                    crate::generators::sqlite::SQL_KIND::SELECT,
                )
                .unwrap_or_else(|| "SELECT 1;".to_string());
            } else if r < prob.SELECT + prob.INSERT {
                return crate::generators::sqlite::get_stmt_by_seed(
                    &self.conn,
                    &mut self.rng,
                    crate::generators::sqlite::SQL_KIND::INSERT,
                )
                .unwrap_or_else(|| "SELECT 1;".to_string());
            } else if r < prob.SELECT + prob.INSERT + prob.UPDATE {
                return crate::generators::sqlite::get_stmt_by_seed(
                    &self.conn,
                    &mut self.rng,
                    crate::generators::sqlite::SQL_KIND::UPDATE,
                )
                .unwrap_or_else(|| "SELECT 1;".to_string());
            } else {
                return crate::generators::sqlite::get_stmt_by_seed(
                    &self.conn,
                    &mut self.rng,
                    crate::generators::sqlite::SQL_KIND::VACUUM,
                )
                .unwrap_or_else(|| "SELECT 1;".to_string());
            }
        }
        "SELECT 1;".to_string()
    }
}