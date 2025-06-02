use rusqlite::Connection;
use crate::utils::rand_by_seed::LcgRng;
use crate::generators::sqlite;

pub struct Engine {
    pub rng: LcgRng,
}

impl Engine {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: LcgRng::new(seed),
        }
    }

    pub fn next_sql(&mut self, conn: &Connection) -> Option<String> {
        sqlite::select_stmt::get_stmt_by_seed(conn, &mut self.rng)
    }
}