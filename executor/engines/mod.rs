use rusqlite::Connection;
use sqlsmith_rs_drivers::{DatabaseDriver, DRIVER_KIND, new_conn};
use sqlsmith_rs_common::rand_by_seed::LcgRng;
use sqlsmith_rs_common::profile::Profile;
use sqlsmith_rs_drivers::limbo_in_mem::LimboDriver;
use crate::generators::common::SqlKind;

// Define Engine trait
pub trait Engine {
    fn run(&mut self);
    fn generate_sql(&mut self) -> String;
    fn get_driver_kind(&self) -> DRIVER_KIND;
    fn get_sqlite_driver_box(&mut self) -> Option<&mut dyn DatabaseDriver<Connection = rusqlite::Connection>>;
    fn get_limbo_driver_box(&mut self) -> Option<&mut dyn DatabaseDriver<Connection = limbo::Connection>>;
}

// For SQLite driver implementation
pub struct SqliteEngine<'a> {
    pub rng: LcgRng,
    pub sqlite_driver_box: Box<dyn DatabaseDriver<Connection = Connection> + 'a>,
    pub run_count: usize,
    pub stmt_prob: Option<sqlsmith_rs_common::profile::StmtProb>,
    pub debug: Option<sqlsmith_rs_common::profile::DebugOptions>,
}

fn run_engine_loop<F>(run_count: usize, debug: &Option<sqlsmith_rs_common::profile::DebugOptions>, mut gen_and_exec: F)
where
    F: FnMut() -> anyhow::Result<(String, usize)>,
{
    let mut i = 0;
    while i < run_count {
        match gen_and_exec() {
            Ok((sql, affected)) => {
                if let Some(debug) = debug {
                    if debug.show_success_sql {
                        log::info!("SQL executed successfully: {} (affected: {})", sql, affected);
                    }
                }
            }
            Err(e) => {
                if let Some(debug) = debug {
                    if debug.show_failed_sql {
                        log::info!("Error executing SQL: {} with ret: [{:?}]", "<unknown>", e);
                    }
                }
            }
        }
        i += 1;
    }
}

fn generate_sql_by_prob<F>(prob: &sqlsmith_rs_common::profile::StmtProb, rng: &mut sqlsmith_rs_common::rand_by_seed::LcgRng, mut get_stmt: F) -> String
where
    F: FnMut(SqlKind, &mut sqlsmith_rs_common::rand_by_seed::LcgRng) -> Option<String>,
{
    let total = prob.SELECT + prob.INSERT + prob.UPDATE + prob.VACUUM + prob.PRAGMA;
    if total == 0 {
        return "SELECT 1;".to_string();
    }
    let r = (rng.rand().abs() as u64) % total;
    if r < prob.SELECT {
        get_stmt(SqlKind::Select, rng)
    } else if r < prob.SELECT + prob.INSERT {
        get_stmt(SqlKind::Insert, rng)
    } else if r < prob.SELECT + prob.INSERT + prob.UPDATE {
        get_stmt(SqlKind::Update, rng)
    } else if r < prob.SELECT + prob.INSERT + prob.UPDATE + prob.VACUUM {
        get_stmt(SqlKind::Vacuum, rng)
    } else {
        get_stmt(SqlKind::Pragma, rng)
    }
    .unwrap_or_else(|| "SELECT 1;".to_string())
}

impl<'a> Engine for SqliteEngine<'a> {
    fn run(&mut self) {
        let debug = &self.debug;
        let prob = &self.stmt_prob;
        let run_count = self.run_count;
        let rng = &mut self.rng;
        for _ in 0..run_count {
            let conn = self.sqlite_driver_box.get_connection_mut();
            let sql = if let Some(prob) = prob {
                generate_sql_by_prob(prob, rng, |kind, rng| {
                    crate::generators::sqlite::get_stmt_by_seed(conn, rng, kind)
                })
            } else {
                "SELECT 1;".to_string()
            };
            let result = self.sqlite_driver_box.exec(&sql);
            match result {
                Ok(affected) => {
                    if let Some(debug) = debug {
                        if debug.show_success_sql {
                            log::info!("SQL executed successfully: {} (affected: {})", sql, affected);
                        }
                    }
                }
                Err(e) => {
                    if let Some(debug) = debug {
                        if debug.show_failed_sql {
                            log::info!("Error executing SQL: {} with ret: [{:?}]", sql, e);
                        }
                    }
                }
            }
        }
    }

    fn generate_sql(&mut self) -> String {
        let conn = self.sqlite_driver_box.get_connection_mut();
        let rng = &mut self.rng;
        if let Some(prob) = &self.stmt_prob {
            generate_sql_by_prob(prob, rng, |kind, rng| {
                crate::generators::sqlite::get_stmt_by_seed(conn, rng, kind)
            })
        } else {
            "SELECT 1;".to_string()
        }
    }
    fn get_driver_kind(&self) -> DRIVER_KIND {
        DRIVER_KIND::SQLITE_IN_MEM
    }
    // 修改方法名
    fn get_sqlite_driver_box(&mut self) -> Option<&mut dyn DatabaseDriver<Connection = rusqlite::Connection>> {
        Some(&mut *self.sqlite_driver_box) // 返回可变引用
    }
    // New implementation for Limbo driver (always None)
    fn get_limbo_driver_box(&mut self) -> Option<&mut dyn DatabaseDriver<Connection = limbo::Connection>> {
        None
    }
}

pub struct LimboEngine {
    pub rng: LcgRng,
    pub limbo_driver_box: Box<LimboDriver>,
    pub run_count: usize,
    pub stmt_prob: Option<sqlsmith_rs_common::profile::StmtProb>,
    pub debug: Option<sqlsmith_rs_common::profile::DebugOptions>,
}

impl Engine for LimboEngine {
    fn run(&mut self) {
        let debug = &self.debug;
        let prob = &self.stmt_prob;
        let run_count = self.run_count;
        let rng = &mut self.rng;
        for _ in 0..run_count {
            let conn = self.limbo_driver_box.get_connection_mut();
            let sql = if let Some(prob) = prob {
                generate_sql_by_prob(prob, rng, |kind, rng| {
                    crate::generators::limbo::get_stmt_by_seed(conn, rng, kind)
                })
            } else {
                "SELECT 1;".to_string()
            };
            let result = self.limbo_driver_box.exec(&sql);
            match result {
                Ok(affected) => {
                    if let Some(debug) = debug {
                        if debug.show_success_sql {
                            log::info!("SQL executed successfully: {} (affected: {})", sql, affected);
                        }
                    }
                }
                Err(e) => {
                    if let Some(debug) = debug {
                        if debug.show_failed_sql {
                            log::info!("Error executing SQL: {} with ret: [{:?}]", sql, e);
                        }
                    }
                }
            }
        }
    }

    fn generate_sql(&mut self) -> String {
        let conn = self.limbo_driver_box.get_connection_mut();
        let rng = &mut self.rng;
        if let Some(prob) = &self.stmt_prob {
            generate_sql_by_prob(prob, rng, |kind, rng| {
                crate::generators::limbo::get_stmt_by_seed(conn, rng, kind)
            })
        } else {
            "SELECT 1;".to_string()
        }
    }
    fn get_driver_kind(&self) -> DRIVER_KIND {
        DRIVER_KIND::LIMBO_IN_MEM
    }
    fn get_sqlite_driver_box(&mut self) -> Option<&mut dyn DatabaseDriver<Connection = Connection>> {
        None
    }
    // New implementation for Limbo driver
    fn get_limbo_driver_box(&mut self) -> Option<&mut dyn DatabaseDriver<Connection = limbo::Connection>> {
        Some(&mut *self.limbo_driver_box) // Now matches LimboDriver's DatabaseDriver implementation
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
        DRIVER_KIND::LIMBO_IN_MEM => {
            let rt = tokio::runtime::Runtime::new()?;
            let driver = rt.block_on(sqlsmith_rs_drivers::limbo_in_mem::LimboDriver::new())?;
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