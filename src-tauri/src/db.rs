use std::path::Path;

use rusqlite::Connection;

use crate::models::{DaySummary, DailyStats, Session, SessionType, Tag, TagSummary, WeeklyStats};

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

fn session_type_to_db(session_type: &SessionType) -> &'static str {
    match session_type {
        SessionType::Work => "work",
        SessionType::ShortBreak => "short_break",
        SessionType::LongBreak => "long_break",
    }
}

fn session_type_from_db(s: &str) -> SessionType {
    match s {
        "short_break" => SessionType::ShortBreak,
        "long_break" => SessionType::LongBreak,
        _ => SessionType::Work,
    }
}

fn session_from_row(row: &rusqlite::Row) -> Result<Session, rusqlite::Error> {
    let type_str: String = row.get(4)?;
    Ok(Session {
        id: row.get(0)?,
        start_time: row.get(1)?,
        end_time: row.get(2)?,
        duration_s: row.get::<_, i32>(3)? as u32,
        session_type: session_type_from_db(&type_str),
        tag_id: row.get(5)?,
        completed: row.get::<_, i32>(6)? != 0,
        created_at: row.get(7)?,
    })
}

fn tag_from_row(row: &rusqlite::Row) -> Result<Tag, rusqlite::Error> {
    Ok(Tag {
        id: row.get(0)?,
        name: row.get(1)?,
        color: row.get(2)?,
        created_at: row.get(3)?,
    })
}

pub fn create_tag(app_data_dir: &Path, name: &str, color: &str) -> Result<Tag, rusqlite::Error> {
    let conn = Connection::open(app_data_dir.join(DB_NAME))?;
    conn.execute(
        "INSERT INTO tags (name, color) VALUES (?1, ?2)",
        rusqlite::params![name, color],
    )?;
    let id = conn.last_insert_rowid();
    let tag = conn.query_row("SELECT id, name, color, created_at FROM tags WHERE id = ?1", [id], tag_from_row)?;
    Ok(tag)
}

pub fn get_tags(app_data_dir: &Path) -> Result<Vec<Tag>, rusqlite::Error> {
    let conn = Connection::open(app_data_dir.join(DB_NAME))?;
    let mut stmt = conn.prepare("SELECT id, name, color, created_at FROM tags ORDER BY name")?;
    let tags: Vec<Tag> = stmt.query_map([], tag_from_row)?.collect::<Result<Vec<_>, _>>()?;
    Ok(tags)
}

pub fn update_tag(app_data_dir: &Path, id: i64, name: &str, color: &str) -> Result<Tag, rusqlite::Error> {
    let conn = Connection::open(app_data_dir.join(DB_NAME))?;
    conn.execute("UPDATE tags SET name = ?1, color = ?2 WHERE id = ?3", rusqlite::params![name, color, id])?;
    conn.query_row("SELECT id, name, color, created_at FROM tags WHERE id = ?1", [id], tag_from_row)
}

pub fn delete_tag(app_data_dir: &Path, id: i64) -> Result<(), rusqlite::Error> {
    let conn = Connection::open(app_data_dir.join(DB_NAME))?;
    conn.execute("DELETE FROM tags WHERE id = ?1", [id])?;
    Ok(())
}

pub fn get_daily_stats(app_data_dir: &Path, date: &str) -> Result<DailyStats, rusqlite::Error> {
    let conn = Connection::open(app_data_dir.join(DB_NAME))?;
    let (total_secs, session_count): (i64, i64) = conn.query_row(
        "SELECT COALESCE(SUM(duration_s), 0), COUNT(*) FROM sessions
         WHERE type = 'work' AND date(start_time) = ?1 AND completed = 1",
        [date],
        |r| Ok((r.get(0)?, r.get(1)?)),
    )?;

    let mut stmt = conn.prepare(
        "SELECT t.name, t.color, COALESCE(SUM(s.duration_s), 0) as total
         FROM tags t
         JOIN sessions s ON s.tag_id = t.id
         WHERE s.type = 'work' AND date(s.start_time) = ?1 AND s.completed = 1
         GROUP BY t.id",
    )?;
    let sessions_by_tag: Vec<TagSummary> = stmt
        .query_map([date], |r| {
            Ok(TagSummary {
                tag_name: r.get(0)?,
                tag_color: r.get(1)?,
                total_secs: r.get::<_, i32>(2)? as u32,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(DailyStats {
        total_focus_secs: total_secs as u32,
        session_count: session_count as u32,
        sessions_by_tag,
    })
}

pub fn get_weekly_stats(app_data_dir: &Path, week_start: &str) -> Result<WeeklyStats, rusqlite::Error> {
    let conn = Connection::open(app_data_dir.join(DB_NAME))?;
    let total_secs: i64 = conn.query_row(
        "SELECT COALESCE(SUM(duration_s), 0) FROM sessions
         WHERE type = 'work' AND completed = 1
         AND date(start_time) >= ?1 AND date(start_time) < date(?1, '+7 days')",
        [week_start],
        |r| r.get(0),
    )?;

    let mut stmt = conn.prepare(
        "SELECT date(start_time) as d, COALESCE(SUM(duration_s), 0) as total
         FROM sessions
         WHERE type = 'work' AND completed = 1
         AND date(start_time) >= ?1 AND date(start_time) < date(?1, '+7 days')
         GROUP BY d ORDER BY d",
    )?;
    let daily_breakdown: Vec<DaySummary> = stmt
        .query_map([week_start], |r| {
            Ok(DaySummary {
                date: r.get(0)?,
                total_secs: r.get::<_, i32>(1)? as u32,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    let average_per_day = (total_secs as u32) / 7;

    Ok(WeeklyStats {
        total_focus_secs: total_secs as u32,
        daily_breakdown,
        average_per_day,
    })
}

pub fn get_session_history(
    app_data_dir: &Path,
    limit: i32,
    offset: i32,
    tag_id: Option<i64>,
) -> Result<Vec<Session>, rusqlite::Error> {
    let conn = Connection::open(app_data_dir.join(DB_NAME))?;
    let sessions: Vec<Session> = match tag_id {
        Some(id) => {
            let mut stmt = conn.prepare(
                "SELECT id, start_time, end_time, duration_s, type, tag_id, completed, created_at
                 FROM sessions WHERE tag_id = ?1 ORDER BY start_time DESC LIMIT ?2 OFFSET ?3",
            )?;
            let rows: Vec<Session> = stmt
                .query_map(rusqlite::params![id, limit, offset], session_from_row)?
                .collect::<Result<Vec<_>, _>>()?;
            rows
        }
        None => {
            let mut stmt = conn.prepare(
                "SELECT id, start_time, end_time, duration_s, type, tag_id, completed, created_at
                 FROM sessions ORDER BY start_time DESC LIMIT ?1 OFFSET ?2",
            )?;
            let rows: Vec<Session> = stmt
                .query_map(rusqlite::params![limit, offset], session_from_row)?
                .collect::<Result<Vec<_>, _>>()?;
            rows
        }
    };
    Ok(sessions)
}

pub fn get_today_session_count(app_data_dir: &Path) -> Result<u32, rusqlite::Error> {
    let conn = Connection::open(app_data_dir.join(DB_NAME))?;
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sessions WHERE type = 'work' AND date(start_time) = date('now') AND completed = 1",
        [],
        |r| r.get(0),
    )?;
    Ok(count as u32)
}

pub fn insert_session(
    app_data_dir: &Path,
    start_time: &str,
    end_time: Option<&str>,
    duration_s: u32,
    session_type: &SessionType,
    tag_id: Option<i64>,
    completed: bool,
) -> Result<(), rusqlite::Error> {
    let conn = Connection::open(app_data_dir.join(DB_NAME))?;
    conn.execute(
        "INSERT INTO sessions (start_time, end_time, duration_s, type, tag_id, completed)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        rusqlite::params![
            start_time,
            end_time,
            duration_s as i32,
            session_type_to_db(session_type),
            tag_id,
            completed as i32,
        ],
    )?;
    Ok(())
}
