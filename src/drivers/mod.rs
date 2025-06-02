use anyhow::Result;

pub mod sqlite_in_mem;

pub enum DRIVER_KIND {
    SQLITE_IN_MEM,
}

pub trait DatabaseDriver {
    /// The associated type for the database connection object.
    /// This allows `connect()` to return the specific connection type for each driver.
    type Connection;

    /// Establishes a connection to the database.
    ///
    /// # Returns
    /// A `Result` containing the specific connection object or an `anyhow::Error`.
    fn connect(&self) -> Result<Self::Connection>;

    /// Initializes the database schema (e.g., creates tables).
    /// This method takes a mutable reference to an established connection.
    ///
    /// # Arguments
    /// * `conn` - A mutable reference to the database connection.
    ///
    /// # Returns
    /// A `Result` indicating success or failure.
    fn init(&self, conn: &mut Self::Connection) -> Result<()>;

    fn exec(&self, conn: &mut Self::Connection, sql: &str) -> Result<usize>;
}