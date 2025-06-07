# sqlsmith-rs

`sqlsmith-rs` is a SQL testing tool written in Rust. It can generate and execute random SQL statements, with support for SQLite in-memory databases and Limbo databases. The current version of the project is v0.1.2.

## Version Information
### v0.1.2
- **New Features**:
  - **PRAGMA Support**: Added support for PRAGMA statements generation in both SQLite and Limbo drivers.
  - **Enhanced Connection Handling**: Improved connection management and error handling in database drivers.
- **Improvements**:
  - **Unified Interface**: Enhanced driver interface unification for better cross-database compatibility.
  - **Connection Type Safety**: Improved type safety in database connection handling.
  - **Better Error Handling**: Enhanced error reporting and handling across drivers.

### v0.1.1
- **New Features**:
  - **Limbo Database Support**: Added support for Limbo database driver (`LIMBO`), including async SQL generation and execution.
  - **Unified SQL Generation**: All SQL generation logic now supports both SQLite and Limbo via a unified interface.
  - **Async Engine**: The engine and SQL generation functions now support async/await for better compatibility with async database drivers.
  - **Improved Schema Discovery**: Limbo schema discovery now uses async queries and pragma introspection.
- **Improvements**:
  - Refactored code to reduce duplication between drivers.
  - Improved error handling and logging.

### v0.1.0
- **Core Features**:
  - **Database Driver**: Supports SQLite in-memory database (`SQLITE_IN_MEM`).
  - **SQL Generation**: Randomly generates `SELECT`, `INSERT`, `UPDATE`, and `VACUUM` SQL statements based on the configuration file.
  - **Probability Configuration**: Allows users to configure the generation probability of different types of SQL statements.
  - **Debugging Options**: Provides debugging options to choose whether to display successful or failed SQL statements during execution.

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