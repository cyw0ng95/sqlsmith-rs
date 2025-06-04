use rusqlite::Connection;
use crate::utils::rand_by_seed::LcgRng;

pub fn get_vacuum_stmt_by_seed(_sqlite_conn: &Connection, _rng: &mut LcgRng) -> Option<String> {
    // VACUUM 语句不需要连接或随机数种子，所以参数可以忽略
    Some("VACUUM;".to_string())
}