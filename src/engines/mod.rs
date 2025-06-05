use rusqlite::Connection;
use crate::drivers::{DatabaseDriver, DRIVER_KIND, new_conn};
use crate::utils::rand_by_seed::LcgRng;
use crate::profile::Profile;
use crate::drivers::limbo::LimboDriver;

pub struct Engine<'a> {
    pub rng: LcgRng,
    pub driver_kind: DRIVER_KIND,
    pub sqlite_driver_box: Option<Box<dyn DatabaseDriver<Connection = Connection> + 'a>>,
    pub limbo_driver_box: Option<Box<LimboDriver>>,
    pub run_count: usize,
    pub stmt_prob: Option<crate::profile::StmtProb>,
    pub debug: Option<crate::profile::DebugOptions>,
}

impl<'a> Engine<'a> {
    pub fn with_driver_kind(
        seed: u64,
        kind: DRIVER_KIND,
        run_count: usize,
        profile: &Profile,
    ) -> anyhow::Result<Self> {
        let (sqlite_driver_box, limbo_driver_box) = match kind {
            DRIVER_KIND::SQLITE_IN_MEM => {
                let driver = new_conn(DRIVER_KIND::SQLITE_IN_MEM)?;
                (Some(driver), None)
            }
            DRIVER_KIND::LIMBO => {
                let rt = tokio::runtime::Runtime::new()?;
                let driver = rt.block_on(crate::drivers::limbo::LimboDriver::new())?;
                (None, Some(Box::new(driver)))
            }
        };
        Ok(Self {
            rng: LcgRng::new(seed),
            driver_kind: kind,
            sqlite_driver_box,
            limbo_driver_box,
            run_count,
            stmt_prob: profile.stmt_prob.clone(),
            debug: profile.debug.clone(),
        })
    }

    pub fn run(&mut self) {
        let mut i = 0;
        while i < self.run_count {
            let sql = self.generate_sql();
            let result = match self.driver_kind {
                DRIVER_KIND::SQLITE_IN_MEM => {
                    if let Some(driver) = &mut self.sqlite_driver_box {
                        if sql.trim_start().to_uppercase().starts_with("SELECT") {
                            driver.query(&sql)
                        } else {
                            driver.exec(&sql)
                        }
                    } else {
                        Err(anyhow::anyhow!("No sqlite driver"))
                    }
                }
                DRIVER_KIND::LIMBO => {
                    if let Some(driver) = &self.limbo_driver_box {
                        if sql.trim_start().to_uppercase().starts_with("SELECT") {
                            driver.query(&sql)
                        } else {
                            driver.exec(&sql)
                        }
                    } else {
                        Err(anyhow::anyhow!("No limbo driver"))
                    }
                }
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

            match self.driver_kind {
                DRIVER_KIND::SQLITE_IN_MEM => {
                    let conn = self.sqlite_driver_box
                        .as_mut()
                        .map(|d| d.get_connection_mut())
                        .unwrap();
                    if r < prob.SELECT {
                        crate::generators::sqlite::get_stmt_by_seed(
                            conn,
                            &mut self.rng,
                            crate::generators::sqlite::SQL_KIND::SELECT,
                        ).unwrap_or_else(|| "SELECT 1;".to_string())
                    } else if r < prob.SELECT + prob.INSERT {
                        crate::generators::sqlite::get_stmt_by_seed(
                            conn,
                            &mut self.rng,
                            crate::generators::sqlite::SQL_KIND::INSERT,
                        ).unwrap_or_else(|| "SELECT 1;".to_string())
                    } else if r < prob.SELECT + prob.INSERT + prob.UPDATE {
                        crate::generators::sqlite::get_stmt_by_seed(
                            conn,
                            &mut self.rng,
                            crate::generators::sqlite::SQL_KIND::UPDATE,
                        ).unwrap_or_else(|| "SELECT 1;".to_string())
                    } else {
                        crate::generators::sqlite::get_stmt_by_seed(
                            conn,
                            &mut self.rng,
                            crate::generators::sqlite::SQL_KIND::VACUUM,
                        ).unwrap_or_else(|| "SELECT 1;".to_string())
                    }
                }
                DRIVER_KIND::LIMBO => {
                    // LIMBO 直接返回简单 SQL 或调用专用生成器
                    "SELECT 1;".to_string()
                }
            }
        } else {
            "SELECT 1;".to_string()
        }
    }
}