use sqlsmith_rs_common::rand_by_seed::LcgRng;
use sqlsmith_rs_drivers::limbo_in_mem::LimboDriver;
use sqlsmith_rs_drivers::{DRIVER_KIND, DatabaseDriver};
use log::info;

pub struct LimboEngine {
    pub rng: LcgRng,
    pub limbo_driver_box: Box<LimboDriver>,
    pub run_count: usize,
    pub thread_per_exec: usize,
    pub stmt_prob: Option<sqlsmith_rs_common::profile::StmtProb>,
    pub debug: Option<sqlsmith_rs_common::profile::DebugOptions>,
}

impl super::Engine for LimboEngine {
    fn run(&mut self) {
        use std::sync::Arc;
        use std::thread;

        let (debug, run_count, thread_per_exec) = (
            self.debug.clone(),
            self.run_count,
            self.thread_per_exec,
        );

        // Shared statistics
        let (success_count, failed_new_count) = (
            Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            Arc::new(std::sync::atomic::AtomicUsize::new(0)),
        );

        let start_time = std::time::Instant::now();
        let mut handles = vec![];

        // Divide run_count among threads as evenly as possible
        let base_per_thread = run_count / thread_per_exec;
        let extra = run_count % thread_per_exec;

        for n in 0..thread_per_exec {
            let thread_run_count = if n < extra { base_per_thread + 1 } else { base_per_thread };
            let (success_count, failed_new_count) = (
                Arc::clone(&success_count),
                Arc::clone(&failed_new_count),
            );
            let debug = debug.clone();

            handles.push(thread::spawn(move || {
                let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
                let mut driver = rt.block_on(async {
                    LimboDriver::new().await.expect("Failed to create Limbo driver")
                });

                for _ in 0..thread_run_count {
                    let sql = "SELECT 1;".to_string();

                    match driver.exec(&sql) {
                        Ok(affected) => {
                            if let Some(debug) = &debug {
                                if debug.show_success_sql {
                                    log::info!("SQL executed successfully: {} (affected: {})", sql, affected);
                                }
                            }
                            success_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        }
                        Err(e) => {
                            failed_new_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            if let Some(debug) = &debug {
                                if debug.show_failed_sql {
                                    log::info!("Error executing SQL: {} with ret: [{:?}]", sql, e);
                                }
                            }
                        }
                    }
                }
            }));
        }

        for handle in handles {
            handle.join().expect("Thread panicked");
        }

        let elapsed = start_time.elapsed();
        let (final_success, final_failed_new) = (
            success_count.load(std::sync::atomic::Ordering::Relaxed),
            failed_new_count.load(std::sync::atomic::Ordering::Relaxed)
        );

        info!(
            "finish exec in {:.2?}, success/failed_new: {}/{}",
            elapsed, final_success, final_failed_new
        );
    }

    fn generate_sql(&mut self) -> String {
        "SELECT 1;".to_string()
    }

    fn get_driver_kind(&self) -> DRIVER_KIND { DRIVER_KIND::LIMBO_IN_MEM }
    fn get_sqlite_driver_box(&mut self) -> Option<&mut dyn DatabaseDriver<Connection = rusqlite::Connection>> { None }
    fn get_limbo_driver_box(&mut self) -> Option<&mut dyn DatabaseDriver<Connection = limbo::Connection>> {
        Some(&mut *self.limbo_driver_box)
    }
}
