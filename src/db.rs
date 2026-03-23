use rusqlite::{params, Connection, Result};
use std::path::Path;

const DB_DIR: &str = ".forge";
const DB_FILE: &str = "forge.db";

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ProjectRecord {
    pub id: i64,
    pub name: String,
    pub path: String,
    pub created_at: String,
}

pub fn db_path() -> String {
    format!("{}/{}", DB_DIR, DB_FILE)
}

pub fn ensure_db() -> Result<Connection> {
    if !Path::new(DB_DIR).exists() {
        std::fs::create_dir_all(DB_DIR)
            .map_err(|_| rusqlite::Error::InvalidPath(Path::new(DB_DIR).to_path_buf()))?;
    }

    let conn = Connection::open(db_path())?;
    init_schema(&conn)?;
    Ok(conn)
}

fn init_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS projects (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            name        TEXT NOT NULL UNIQUE,
            path        TEXT NOT NULL,
            created_at  TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS milestones (
            id           INTEGER PRIMARY KEY AUTOINCREMENT,
            project_id   INTEGER NOT NULL,
            name         TEXT NOT NULL,
            status       TEXT NOT NULL,
            created_at   TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at   TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY(project_id) REFERENCES projects(id)
        );

        CREATE TABLE IF NOT EXISTS logs (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            project_id  INTEGER,
            level       TEXT NOT NULL,
            message     TEXT NOT NULL,
            created_at  TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY(project_id) REFERENCES projects(id)
        );
        "
    )?;

    Ok(())
}

pub fn create_project(conn: &Connection, name: &str, path: &str) -> Result<i64> {
    conn.execute(
        "INSERT INTO projects (name, path) VALUES (?1, ?2)",
        params![name, path],
    )?;

    Ok(conn.last_insert_rowid())
}

pub fn get_latest_project(conn: &Connection) -> Result<Option<ProjectRecord>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, path, created_at FROM projects ORDER BY id DESC LIMIT 1",
    )?;

    let mut rows = stmt.query([])?;
    if let Some(row) = rows.next()? {
        Ok(Some(ProjectRecord {
            id: row.get(0)?,
            name: row.get(1)?,
            path: row.get(2)?,
            created_at: row.get(3)?,
        }))
    } else {
        Ok(None)
    }
}

pub fn create_default_milestones(conn: &Connection, project_id: i64) -> Result<()> {
    let milestones = [
        "Initialize project workspace",
        "Create config and prompt files",
        "Verify Git repository",
        "Run initial validation hook",
        "Record first automated commit",
    ];

    for milestone in milestones {
        conn.execute(
            "INSERT INTO milestones (project_id, name, status) VALUES (?1, ?2, 'pending')",
            params![project_id, milestone],
        )?;
    }

    Ok(())
}

pub fn get_next_milestone(conn: &Connection, project_id: i64) -> Result<Option<(i64, String, String)>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, status FROM milestones WHERE project_id = ?1 AND status = 'pending' ORDER BY id ASC LIMIT 1",
    )?;

    let mut rows = stmt.query(params![project_id])?;
    if let Some(row) = rows.next()? {
        Ok(Some((row.get(0)?, row.get(1)?, row.get(2)?)))
    } else {
        Ok(None)
    }
}

pub fn mark_milestone_done(conn: &Connection, milestone_id: i64) -> Result<()> {
    conn.execute(
        "UPDATE milestones SET status = 'done', updated_at = CURRENT_TIMESTAMP WHERE id = ?1",
        params![milestone_id],
    )?;
    Ok(())
}

pub fn add_log(conn: &Connection, project_id: Option<i64>, level: &str, message: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO logs (project_id, level, message) VALUES (?1, ?2, ?3)",
        params![project_id, level, message],
    )?;
    Ok(())
}

pub fn get_recent_logs(conn: &Connection, limit: i64) -> Result<Vec<(String, String, String)>> {
    let mut stmt = conn.prepare(
        "SELECT level, message, created_at FROM logs ORDER BY id DESC LIMIT ?1",
    )?;

    let rows = stmt.query_map(params![limit], |row| {
        Ok((row.get(0)?, row.get(1)?, row.get(2)?))
    })?;

    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}
