use rusqlite::Connection;
use crate::drivers::{DatabaseDriver, DRIVER_KIND, new_conn};
use crate::utils::rand_by_seed::LcgRng;
use crate::profile::Profile;

pub struct Engine<'a> {
    pub rng: LcgRng,
    pub driver_kind: DRIVER_KIND,
    pub driver: Box<dyn DatabaseDriver<Connection = Connection> + 'a>,
    pub run_count: usize,
    pub stmt_prob: Option<crate::profile::StmtProb>,
    pub debug: Option<crate::profile::DebugOptions>,
}

impl<'a> Engine<'a> {
    pub fn with_driver_kind(seed: u64, kind: DRIVER_KIND, run_count: usize, profile: &Profile) -> anyhow::Result<Self> {
        let driver = new_conn(kind)?;
        Ok(Self {
            rng: LcgRng::new(seed),
            driver_kind: kind,
            driver,
            run_count,
            stmt_prob: profile.stmt_prob.clone(),
            debug: profile.debug.clone(),
        })
    }

    pub fn run(&mut self) {
        let mut i = 0;
        while i < self.run_count {
            let sql = self.generate_sql();
            
            // 根据 SQL 类型选择执行方法
            let result = if sql.trim_start().to_uppercase().starts_with("SELECT") {
                self.driver.query(&sql)
            } else {
                self.driver.exec(&sql)
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
        if let Some(prob) = &self.stmt_prob {
            let total = prob.SELECT + prob.INSERT + prob.UPDATE + prob.VACUUM;
            if total == 0 {
                return "SELECT 1;".to_string();
            }
            
            let r = (self.rng.rand().abs() as u64) % total;
            let conn = self.driver.get_connection_mut();
            
            if r < prob.SELECT {
                return crate::generators::sqlite::get_stmt_by_seed(
                    conn,
                    &mut self.rng,
                    crate::generators::sqlite::SQL_KIND::SELECT,
                )
                .unwrap_or_else(|| "SELECT 1;".to_string());
            } else if r < prob.SELECT + prob.INSERT {
                return crate::generators::sqlite::get_stmt_by_seed(
                    conn,
                    &mut self.rng,
                    crate::generators::sqlite::SQL_KIND::INSERT,
                )
                .unwrap_or_else(|| "SELECT 1;".to_string());
            } else if r < prob.SELECT + prob.INSERT + prob.UPDATE {
                return crate::generators::sqlite::get_stmt_by_seed(
                    conn,
                    &mut self.rng,
                    crate::generators::sqlite::SQL_KIND::UPDATE,
                )
                .unwrap_or_else(|| "SELECT 1;".to_string());
            } else {
                return crate::generators::sqlite::get_stmt_by_seed(
                    conn,
                    &mut self.rng,
                    crate::generators::sqlite::SQL_KIND::VACUUM,
                )
                .unwrap_or_else(|| "SELECT 1;".to_string());
            }
        }
        "SELECT 1;".to_string()
    }
}