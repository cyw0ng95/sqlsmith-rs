use rusqlite::Connection;
use crate::utils::rand_by_seed::LcgRng;
use crate::generators::sqlite;
use crate::drivers::DatabaseDriver;

pub struct Engine<'a, D: DatabaseDriver<Connection = Connection>> {
    pub rng: LcgRng,
    pub driver: &'a D,
}

impl<'a, D: DatabaseDriver<Connection = Connection>> Engine<'a, D> {
    pub fn new(seed: u64, driver: &'a D) -> Self {
        Self {
            rng: LcgRng::new(seed),
            driver,
        }
    }

    pub fn next_sql(&mut self, conn: &Connection) -> Option<String> {
        sqlite::select_stmt::get_stmt_by_seed(conn, &mut self.rng)
    }

    pub fn exec(&self, conn: &mut Connection, sql: &str) -> anyhow::Result<usize> {
        self.driver.exec(conn, sql)
    }
}