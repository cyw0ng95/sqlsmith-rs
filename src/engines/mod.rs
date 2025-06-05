use rusqlite::Connection;
use crate::drivers::{DatabaseDriver, DRIVER_KIND, new_conn};
use crate::utils::rand_by_seed::LcgRng;
use crate::profile::Profile;
use crate::drivers::limbo::LimboDriver;

// 定义 Engine trait
pub trait Engine {
    fn run(&mut self);
    fn generate_sql(&mut self) -> String;
    fn get_driver_kind(&self) -> DRIVER_KIND;
    // 修改返回类型
    fn get_sqlite_driver_box(&mut self) -> Option<&mut dyn DatabaseDriver<Connection = Connection>>; // 改为可变引用
}

// 为 SQLite 驱动实现 Engine trait
pub struct SqliteEngine<'a> {
    pub rng: LcgRng,
    pub sqlite_driver_box: Box<dyn DatabaseDriver<Connection = Connection> + 'a>,
    pub run_count: usize,
    pub stmt_prob: Option<crate::profile::StmtProb>,
    pub debug: Option<crate::profile::DebugOptions>,
}

impl<'a> Engine for SqliteEngine<'a> {
    fn run(&mut self) {
        let mut i = 0;
        while i < self.run_count {
            let sql = self.generate_sql();
            let result = self.sqlite_driver_box.exec(&sql);

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

            let conn = self.sqlite_driver_box.get_connection_mut();
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
        } else {
            "SELECT 1;".to_string()
        }
    }
    fn get_driver_kind(&self) -> DRIVER_KIND {
        DRIVER_KIND::SQLITE_IN_MEM
    }
    // 修改方法名
    fn get_sqlite_driver_box(&mut self) -> Option<&mut dyn DatabaseDriver<Connection = Connection>> {
        Some(&mut *self.sqlite_driver_box) // 返回可变引用
    }
}

pub struct LimboEngine {
    pub rng: LcgRng,
    pub limbo_driver_box: Box<LimboDriver>,
    pub run_count: usize,
    pub stmt_prob: Option<crate::profile::StmtProb>,
    pub debug: Option<crate::profile::DebugOptions>,
}

impl Engine for LimboEngine {
    fn run(&mut self) {
        let mut i = 0;
        while i < self.run_count {
            let sql = self.generate_sql();
            let result = self.limbo_driver_box.exec(&sql);

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
            // 目前 Limbo 直接返回简单 SQL 或调用专用生成器
            "SELECT 1;".to_string()
        } else {
            "SELECT 1;".to_string()
        }
    }
    fn get_driver_kind(&self) -> DRIVER_KIND {
        DRIVER_KIND::LIMBO
    }
    fn get_sqlite_driver_box(&mut self) -> Option<&mut dyn DatabaseDriver<Connection = Connection>> {
        None
    }
}

// 修改 with_driver_kind 函数
pub fn with_driver_kind(
    seed: u64,
    kind: DRIVER_KIND,
    run_count: usize,
    profile: &Profile,
) -> anyhow::Result<Box<dyn Engine>> {
    match kind {
        DRIVER_KIND::SQLITE_IN_MEM => {
            let driver = new_conn(DRIVER_KIND::SQLITE_IN_MEM)?;
            Ok(Box::new(SqliteEngine {
                rng: LcgRng::new(seed),
                sqlite_driver_box: driver,
                run_count,
                stmt_prob: profile.stmt_prob.clone(),
                debug: profile.debug.clone(),
            }))
        }
        DRIVER_KIND::LIMBO => {
            let rt = tokio::runtime::Runtime::new()?;
            let driver = rt.block_on(crate::drivers::limbo::LimboDriver::new())?;
            Ok(Box::new(LimboEngine {
                rng: LcgRng::new(seed),
                limbo_driver_box: Box::new(driver),
                run_count,
                stmt_prob: profile.stmt_prob.clone(),
                debug: profile.debug.clone(),
            }))
        }
    }
}