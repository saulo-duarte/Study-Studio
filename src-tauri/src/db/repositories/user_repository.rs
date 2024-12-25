use rusqlite::{params, Connection, Result};
use crate::db::models::{
    user::User,
    user_available_day::DayOfWeek,
};

pub struct UserRepository<'a> {
    conn: &'a mut Connection,
}

impl<'a> UserRepository<'a> {
    pub fn new(conn: &'a mut Connection) -> Self {
        Self { conn }
    }

    pub fn create_user(
        &mut self,
        user: &User,
        available_days: Vec<DayOfWeek>,
        interests: Vec<String>,
    ) -> Result<i32> {
        let tx = self.conn.transaction()?;

        {
            let mut stmt = tx.prepare(
                "INSERT INTO users (name, email, status, created_at, last_login)
                 VALUES (?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)",
            )?;

            stmt.execute(params![
                user.name,
                user.email,
                user.status.as_str()
            ])?;
        }

        let user_id = tx.last_insert_rowid() as i32;

        {
            let mut day_stmt = tx.prepare(
                "INSERT INTO user_available_days (user_id, day_of_week)
                 VALUES (?, ?)",
            )?;

            for day in available_days {
                day_stmt.execute(params![user_id, day.as_str()])?;
            }
        }

        {
            let mut interest_stmt = tx.prepare(
                "INSERT INTO user_interests (user_id, interest)
                 VALUES (?, ?)",
            )?;

            for interest in interests {
                interest_stmt.execute(params![user_id, interest])?;
            }
        }
        tx.commit()?;

        Ok(user_id)
    }

    pub fn check_if_there_is_active_user_status(&mut self) -> Result<bool> {
        self.conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM users WHERE status = 'active')",
            [],
            |row| row.get(0)
        )
    }
}
