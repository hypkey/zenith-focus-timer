use std::path::Path;

use rusqlite::Connection;

const DB_NAME: &str = "zenith.db";

const MIGRATION_1: &str = "
CREATE TABLE IF NOT EXISTS tags (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT NOT NULL UNIQUE,
    color       TEXT NOT NULL,
    created_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS sessions (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    start_time  TEXT NOT NULL,
    end_time    TEXT,
    duration_s  INTEGER NOT NULL,
    type        TEXT NOT NULL,
    tag_id      INTEGER,
    completed   INTEGER NOT NULL DEFAULT 0,
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE SET NULL
);
";

pub fn init_db(app_data_dir: &Path) -> Result<Connection, rusqlite::Error> {
    std::fs::create_dir_all(app_data_dir)
        .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;
    let db_path = app_data_dir.join(DB_NAME);
    let conn = Connection::open(db_path)?;
    conn.execute_batch(MIGRATION_1)?;
    Ok(conn)
}

#[allow(dead_code)]
pub fn db_path(app_data_dir: &Path) -> std::path::PathBuf {
    app_data_dir.join(DB_NAME)
}
