use crate::generators::common::SqlKind;
use log::info;
use rusqlite::Connection;
use sqlsmith_rs_common::rand_by_seed::LcgRng;
use sqlsmith_rs_drivers::{DRIVER_KIND, DatabaseDriver, new_conn};

pub struct SqliteEngine<'a> {
    pub rng: LcgRng,
    pub sqlite_driver_box: Box<dyn DatabaseDriver<Connection = Connection> + 'a>,
    pub run_count: usize,
    pub thread_per_exec: usize,
    pub stmt_prob: Option<sqlsmith_rs_common::profile::StmtProb>,
    pub debug: Option<sqlsmith_rs_common::profile::DebugOptions>,
}

fn generate_sql_by_prob<F>(
    prob: &sqlsmith_rs_common::profile::StmtProb,
    rng: &mut LcgRng,
    mut get_stmt: F,
) -> String
where
    F: FnMut(SqlKind, &mut LcgRng) -> Option<String>,
{
    let thresholds = [
        (prob.SELECT, SqlKind::Select),
        (prob.INSERT, SqlKind::Insert),
        (prob.UPDATE, SqlKind::Update),
        (prob.DELETE, SqlKind::Delete),
        (prob.VACUUM, SqlKind::Vacuum),
        (prob.PRAGMA, SqlKind::Pragma),
        (prob.CREATE_TRIGGER, SqlKind::CreateTrigger),
        (prob.DROP_TRIGGER, SqlKind::DropTrigger),
    ];

    let total: u64 = thresholds.iter().map(|(p, _)| p).sum();
    if total == 0 {
        return "SELECT 1;".to_string();
    }

    let r = (rng.rand().abs() as u64) % total;
    let mut accum = 0;

    for (prob, kind) in thresholds {
        accum += prob;
        if r < accum {
            return get_stmt(kind, rng).unwrap_or_else(|| "SELECT 1;".to_string());
        }
    }

    "SELECT 1;".to_string()
}

impl<'a> super::Engine for SqliteEngine<'a> {
    fn run(&mut self) {
        use std::sync::{Arc, Mutex};
        use std::thread;

        let (debug, prob, run_count, thread_per_exec, base_seed) = (
            self.debug.clone(),
            self.stmt_prob.clone(),
            self.run_count,
            self.thread_per_exec,
            self.rng.get_seed()
        );

        // Shared statistics
        let (success_count, failed_expected_count, failed_new_count, stmt_type_counts) = (
            Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            Arc::new(Mutex::new(std::collections::HashMap::new()))
        );

        let start_time = std::time::Instant::now();
        let mut handles = vec![];

        for n in 0..thread_per_exec {
            let thread_run_count = run_count;
            let (thread_seed, debug, prob) = (
                base_seed.wrapping_add(n as u64),
                debug.clone(),
                prob.clone()
            );
            let (success_count, failed_expected_count, failed_new_count, stmt_type_counts) = (
                Arc::clone(&success_count),
                Arc::clone(&failed_expected_count),
                Arc::clone(&failed_new_count),
                Arc::clone(&stmt_type_counts)
            );

            handles.push(thread::spawn(move || {
                let mut driver = new_conn(DRIVER_KIND::SQLITE_IN_MEM).expect("Failed to create driver");
                let mut rng = LcgRng::new(thread_seed);
                let ignorable_errors = vec![rusqlite::ErrorCode::ConstraintViolation];
                let mut local_stmt_type_counts = std::collections::HashMap::new();

                for _ in 0..thread_run_count {
                    let conn = driver.get_connection_mut();
                    let sql = if let Some(prob) = &prob {
                        generate_sql_by_prob(prob, &mut rng, |kind, rng| {
                            *local_stmt_type_counts.entry(kind.clone()).or_insert(0) += 1;
                            crate::generators::sqlite::get_stmt_by_seed(conn, rng, kind)
                        })
                    } else {
                        "SELECT 1;".to_string()
                    };

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
                            let error_code = if let Some(rusqlite_error) = e.downcast_ref::<rusqlite::Error>() {
                                match rusqlite_error {
                                    rusqlite::Error::SqliteFailure(errcode, _) => errcode.code,
                                    _ => rusqlite::ErrorCode::Unknown,
                                }
                            } else {
                                rusqlite::ErrorCode::Unknown
                            };

                            if !ignorable_errors.contains(&error_code) {
                                failed_new_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                                if let Some(debug) = &debug {
                                    if debug.show_failed_sql {
                                        log::info!("Error executing SQL: {} with ret: [{:?}]", sql, error_code);
                                    }
                                }
                            } else {
                                failed_expected_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            }
                        }
                    }
                }

                // Merge local statement type counts
                if let Ok(mut global_map) = stmt_type_counts.lock() {
                    for (k, v) in local_stmt_type_counts {
                        *global_map.entry(k).or_insert(0) += v;
                    }
                }
            }));
        }

        for handle in handles {
            handle.join().expect("Thread panicked");
        }

        let elapsed = start_time.elapsed();
        let (final_success, final_failed_exp, final_failed_new) = (
            success_count.load(std::sync::atomic::Ordering::Relaxed),
            failed_expected_count.load(std::sync::atomic::Ordering::Relaxed),
            failed_new_count.load(std::sync::atomic::Ordering::Relaxed)
        );

        info!(
            "finish exec in {:.2?}, success/failed_exp/failed_new: {}/{}/{}",
            elapsed, final_success, final_failed_exp, final_failed_new
        );
        
        if let Ok(stmt_type_counts) = stmt_type_counts.lock() {
            info!("Statement type statistics: {:?}", *stmt_type_counts);
        }
    }

    fn generate_sql(&mut self) -> String {
        let conn = self.sqlite_driver_box.get_connection_mut();
        if let Some(prob) = &self.stmt_prob {
            generate_sql_by_prob(prob, &mut self.rng, |kind, rng| {
                crate::generators::sqlite::get_stmt_by_seed(conn, rng, kind)
            })
        } else {
            "SELECT 1;".to_string()
        }
    }

    fn get_driver_kind(&self) -> DRIVER_KIND { DRIVER_KIND::SQLITE_IN_MEM }
    
    fn get_sqlite_driver_box(&mut self) -> Option<&mut dyn DatabaseDriver<Connection = rusqlite::Connection>> {
        Some(&mut *self.sqlite_driver_box)
    }
    
    fn get_limbo_driver_box(&mut self) -> Option<&mut dyn DatabaseDriver<Connection = limbo::Connection>> {
        None
    }
}
