use rusqlite::Connection;
use sqlsmith_rs_common::rand_by_seed::LcgRng;

enum PragmaKind {
    NoArg(&'static str),
    BoolArg(&'static str),
    IntArg(&'static str, i64, i64),
    StringArg(&'static str),
}

pub fn get_pragma_stmt_by_seed(_conn: &Connection, rng: &mut LcgRng) -> Option<String> {
    use PragmaKind::*;
    const PRAGMAS: &[PragmaKind] = &[
        NoArg("integrity_check"),
        NoArg("quick_check"),
        NoArg("foreign_key_check"),
        NoArg("database_list"),
        NoArg("collation_list"),
        NoArg("table_info"),
        NoArg("index_list"),
        NoArg("index_info"),
        NoArg("stats"),
        NoArg("page_count"),
        NoArg("schema_version"),
        NoArg("user_version"),
        NoArg("encoding"),
        BoolArg("foreign_keys"),
        BoolArg("case_sensitive_like"),
        BoolArg("automatic_index"),
        BoolArg("cache_spill"),
        BoolArg("recursive_triggers"),
        BoolArg("journal_size_limit"),
        BoolArg("legacy_file_format"),
        BoolArg("writable_schema"),
        IntArg("cache_size", 100, 10000),
        IntArg("page_size", 512, 65536),
        IntArg("mmap_size", 0, 104857600),
        IntArg("wal_autocheckpoint", 1, 10000),
        StringArg("journal_mode"),
        StringArg("locking_mode"),
        StringArg("synchronous"),
        StringArg("temp_store"),
        StringArg("encoding"),
    ];
    let idx = (rng.rand().unsigned_abs() as usize) % PRAGMAS.len();
    let pragma = &PRAGMAS[idx];
    let sql = match pragma {
        NoArg(name) => format!("PRAGMA {};", name),
        BoolArg(name) => {
            let val = if rng.rand().abs() % 2 == 0 {
                "ON"
            } else {
                "OFF"
            };
            format!("PRAGMA {} = {};", name, val)
        }
        IntArg(name, min, max) => {
            let val = min + (rng.rand().unsigned_abs() as i64) % (max - min + 1);
            format!("PRAGMA {} = {};", name, val)
        }
        StringArg(name) => {
            let val = match *name {
                "journal_mode" => {
                    const MODES: &[&str] =
                        &["DELETE", "TRUNCATE", "PERSIST", "MEMORY", "WAL", "OFF"];
                    MODES[(rng.rand().unsigned_abs() as usize) % MODES.len()]
                }
                "locking_mode" => {
                    const MODES: &[&str] = &["NORMAL", "EXCLUSIVE"];
                    MODES[(rng.rand().unsigned_abs() as usize) % MODES.len()]
                }
                "synchronous" => {
                    const MODES: &[&str] = &["OFF", "NORMAL", "FULL", "EXTRA"];
                    MODES[(rng.rand().unsigned_abs() as usize) % MODES.len()]
                }
                "temp_store" => {
                    const MODES: &[&str] = &["DEFAULT", "FILE", "MEMORY"];
                    MODES[(rng.rand().unsigned_abs() as usize) % MODES.len()]
                }
                "encoding" => {
                    const MODES: &[&str] =
                        &["\"UTF-8\"", "\"UTF-16\"", "\"UTF-16le\"", "\"UTF-16be\""];
                    MODES[(rng.rand().unsigned_abs() as usize) % MODES.len()]
                }
                _ => "ON",
            };
            format!("PRAGMA {} = {};", name, val)
        }
    };
    Some(sql)
}
