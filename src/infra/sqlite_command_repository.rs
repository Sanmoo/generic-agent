use crate::core::ports::{command_repository::CommandRepository, command::Command};

use super::db::Pool;

pub struct SqliteCommandRepository {
    pub pool: Pool
}

impl SqliteCommandRepository {
    pub fn new(pool: Pool) -> Self { Self { pool } }
}

impl CommandRepository for SqliteCommandRepository {
    fn store_new_command(&self, command: &str) -> Result<u32, String> {
        let pool = self.pool.clone();
        let conn = pool.get().map_err(|e| e.to_string())?;

        conn.execute(
            "INSERT INTO commands(created_at, started_at, command) VALUES (date('now'), date('now'), ?)",
            [command]
        ).map_err(|e|e.to_string())?;

        conn.query_row("SELECT last_insert_rowid()", [], |row| row.get(0)).map_err(|e|e.to_string())
    }

    fn fetch_command(&self, command_id: u32) -> Result<crate::core::ports::command::Command, String> {
        let pool = self.pool.clone();
        let conn = pool.get().map_err(|e| e.to_string())?;

        conn.query_row(
            "select * from commands where id = ?",
            [&command_id],
            |row| Ok(Command{
                id: row.get("id")?,
                command: row.get("command")?,
                created_at: row.get("created_at")?,
                finished_at: row.get("finished_at")?,
                started_at: row.get("started_at")?,
                stderr: row.get("stderr")?,
                stdout: row.get("stdout")?,
                status: row.get("status")?
            }),
        ).map_err(|e| e.to_string())
    }
}