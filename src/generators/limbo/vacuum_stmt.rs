use limbo::Connection;
use crate::utils::rand_by_seed::LcgRng;

/// 生成一个简单的 VACUUM 语句
pub fn get_vacuum_stmt_by_seed(_conn: &Connection, _rng: &mut LcgRng) -> Option<String> {
    Some("VACUUM;".to_string())
}