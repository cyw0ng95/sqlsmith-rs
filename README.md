# sqlsmith-rs

`sqlsmith-rs` is a SQL testing tool written in Rust. It can generate and execute random SQL statements, with support for SQLite in-memory databases and Limbo databases. The current version of the project is v0.1.1.

## Version Information
### v0.1.1
- **New Features**:
  - **Limbo Database Support**: Added support for Limbo database driver (`LIMBO`), including async SQL generation and execution.
  - **Unified SQL Generation**: All SQL generation logic now supports both SQLite and Limbo via a unified interface.
  - **Async Engine**: The engine and SQL generation functions now support async/await for better compatibility with async database drivers.
  - **Improved Schema Discovery**: Limbo schema discovery now uses async queries and pragma introspection.
- **Improvements**:
  - Refactored code to reduce duplication between drivers.
  - Improved error handling and logging.
- **Dependencies**:
  - `anyhow`: Used for error handling.
  - `rusqlite`: Used for interacting with SQLite databases.
  - `limbo`: Used for interacting with Limbo databases.
  - `serde` and `serde_json`: Used for serialization and deserialization of configuration files.
  - `tokio`: Used for async runtime support.

### v0.1.0
- **Core Features**:
  - **Database Driver**: Supports SQLite in-memory database (`SQLITE_IN_MEM`).
  - **SQL Generation**: Randomly generates `SELECT`, `INSERT`, `UPDATE`, and `VACUUM` SQL statements based on the configuration file.
  - **Probability Configuration**: Allows users to configure the generation probability of different types of SQL statements.
  - **Debugging Options**: Provides debugging options to choose whether to display successful or failed SQL statements during execution.
- **Dependencies**:
  - `anyhow`: Used for error handling.
  - `rusqlite`: Used for interacting with SQLite databases.
  - `serde` and `serde_json`: Used for serialization and deserialization of configuration files.

## Project Structure
The project has the following basic structure:
```plaintext
sqlsmith-rs/
├── profile.json        # Configuration file
├── src/                # Source code directory
│   ├── engines/        # Engine logic
│   ├── generators/     # SQL statement generation logic
│   ├── profile.rs      # Profile parsing logic
│   └── ...             # Other modules
└── ...                 # Other files
```