use sqlsmith_rs_common::profile::Profile;
use sqlsmith_rs_common::rand_by_seed::LcgRng;
use sqlsmith_rs_drivers::{DRIVER_KIND, DatabaseDriver, new_conn};

mod sqlite_engine;
pub use sqlite_engine::SqliteEngine;

mod limbo_engine;
pub use limbo_engine::LimboEngine;

// Define Engine trait
pub trait Engine {
    fn run(&mut self);
    fn generate_sql(&mut self) -> String;
    fn get_driver_kind(&self) -> DRIVER_KIND;
    fn get_sqlite_driver_box(&mut self) -> Option<&mut dyn DatabaseDriver<Connection = rusqlite::Connection>>;
    fn get_limbo_driver_box(&mut self) -> Option<&mut dyn DatabaseDriver<Connection = limbo::Connection>>;
}

pub fn with_driver_kind(
    seed: u64,
    kind: DRIVER_KIND,
    run_count: usize,
    profile: &Profile,
) -> anyhow::Result<Box<dyn Engine>> {
    let thread_per_exec = profile.thread_per_exec.unwrap_or(5);
    match kind {
        DRIVER_KIND::SQLITE_IN_MEM => {
            let driver = new_conn(DRIVER_KIND::SQLITE_IN_MEM)?;
            Ok(Box::new(SqliteEngine {
                rng: LcgRng::new(seed),
                sqlite_driver_box: driver,
                run_count,
                thread_per_exec,
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
                thread_per_exec,
                stmt_prob: profile.stmt_prob.clone(),
                debug: profile.debug.clone(),
            }))
        }
    }
}
