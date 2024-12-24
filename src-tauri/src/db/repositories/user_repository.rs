use rusqlite::{Connection, Result, params};
use crate::db::models::user::User;
use crate::db::models::user_available_day::{UserAvailableDay, AvailableDay, AvailableDayParseError};
use crate::db::models::user_interesting::UserInterestingField;
use chrono::{DateTime, Utc};
use std::str::FromStr;

pub struct UserRepository<'a> {
    conn: &'a Connection
}

impl<'a> UserRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn create(&self, user: &User) -> Result<User> {
        let sql = "
            INSERT INTO users (name, email, status, created_at, last_login) 
            VALUES (?1, ?2, ?3, ?4, ?5) 
            RETURNING *
        ";
        
        self.conn.query_row(
            sql,
            params![
                user.name,
                user.email,
                user.status,
                user.created_at.to_rfc3339(),
                user.last_login.as_ref().map(|dt| dt.to_rfc3339())
            ],
            |row| {
                Ok(User {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    email: row.get(2)?,
                    status: row.get(3)?,
                    created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                        .unwrap()
                        .with_timezone(&Utc),
                    last_login: row.get::<_, Option<String>>(5)?
                        .map(|dt| DateTime::parse_from_rfc3339(&dt).unwrap().with_timezone(&Utc)),
                })
            }
        )
    }

    pub fn find_by_id(&self, user_id: i32) -> Result<User> {
        let sql = "SELECT * FROM users WHERE id = ?1";
        
        self.conn.query_row(sql, params![user_id], |row| {
            Ok(User {
                id: row.get(0)?,
                name: row.get(1)?,
                email: row.get(2)?,
                status: row.get(3)?,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap()
                    .with_timezone(&Utc),
                last_login: row.get::<_, Option<String>>(5)?
                    .map(|dt| DateTime::parse_from_rfc3339(&dt).unwrap().with_timezone(&Utc)),
            })
        })
    }
    
    pub fn find_all(&self) -> Result<Vec<User>> {
        let sql = "SELECT * FROM users";
        let mut stmt = self.conn.prepare(sql)?;
        
        let users = stmt.query_map([], |row| {
            Ok(User {
                id: row.get(0)?,
                name: row.get(1)?,
                email: row.get(2)?,
                status: row.get(3)?,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap()
                    .with_timezone(&Utc),
                last_login: row.get::<_, Option<String>>(5)?
                    .map(|dt| DateTime::parse_from_rfc3339(&dt).unwrap().with_timezone(&Utc)),
            })
        })?;
    
        users.collect()
    }
    
    pub fn update_status(&self, user_id: i32, new_status: &str) -> Result<User> {
        let sql = "
            UPDATE users 
            SET status = ?1 
            WHERE id = ?2 
            RETURNING *
        ";
        
        self.conn.query_row(sql, params![new_status, user_id], |row| {
            Ok(User {
                id: row.get(0)?,
                name: row.get(1)?,
                email: row.get(2)?,
                status: row.get(3)?,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap()
                    .with_timezone(&Utc),
                last_login: row.get::<_, Option<String>>(5)?
                    .map(|dt| DateTime::parse_from_rfc3339(&dt).unwrap().with_timezone(&Utc)),
            })
        })
    }

    pub fn create_user_available_day(&self, user_available_day: &UserAvailableDay) -> Result<UserAvailableDay> {
        let sql = "
            INSERT INTO user_available_days (user_id, available_day) 
            VALUES (?1, ?2) 
            RETURNING *
        ";
    
        self.conn.query_row(
            sql,
            params![
                user_available_day.user_id,
                user_available_day.available_day.to_string()
            ],
            |row| {
                Ok(UserAvailableDay {
                    user_id: row.get(1)?,
                    available_day: AvailableDay::from_str(&row.get::<_, String>(2)?)
                        .map_err(|e| rusqlite::Error::FromSqlConversionFailure(
                            2,
                            rusqlite::types::Type::Text,
                            Box::new(AvailableDayParseError::InvalidFormat(e.to_string()))
                        ))?,
                })
            }
        )
    }

    pub fn find_user_available_days_by_user_id(&self, user_id: i32) -> Result<Vec<UserAvailableDay>> {
        let sql = "SELECT * FROM user_available_days WHERE user_id = ?1";
        let mut stmt = self.conn.prepare(sql)?;
    
        let available_days = stmt.query_map(params![user_id], |row| {
            let available_day_str: String = row.get(2)?;
            
            Ok(UserAvailableDay {
                user_id: row.get(1)?,
                available_day: AvailableDay::from_str(&available_day_str)
                    .map_err(|e| rusqlite::Error::FromSqlConversionFailure(
                        2, 
                        rusqlite::types::Type::Text, 
                        Box::new(AvailableDayParseError::InvalidFormat(e.to_string()))
                    ))?,
            })
        })?;
    
        available_days.collect()
    }

    pub fn delete_user_available_day(&self, user_id: i32, available_day: &AvailableDay) -> Result<usize> {
        let sql = "DELETE FROM user_available_days WHERE user_id = ?1 AND available_day = ?2";
        self.conn.execute(sql, params![user_id, available_day.to_string()])
    }

    pub fn create_user_interesting_field(
        &self,
        user_interesting_field: &UserInterestingField,
    ) -> Result<UserInterestingField> {
        let sql = "
            INSERT INTO user_interesting_fields (user_id, field) 
            VALUES (?1, ?2) 
            RETURNING *
        ";
    
        self.conn.query_row(
            sql,
            params![
                user_interesting_field.user_id,
                user_interesting_field.field.to_string()
            ],
            |row| {
                Ok(UserInterestingField {
                    user_id: row.get(0)?,
                    field: row.get::<_, String>(1)?
                        .parse() 
                        .map_err(|e| rusqlite::Error::FromSqlConversionFailure(
                            1,
                            rusqlite::types::Type::Text,
                            Box::new(e),
                        ))?,
                })
            },
        )
    }
    
    pub fn find_user_interesting_fields_by_user_id(
        &self,
        user_id: i32,
    ) -> Result<Vec<UserInterestingField>> {
        let sql = "SELECT user_id, field FROM user_interesting_fields WHERE user_id = ?1";
        let mut stmt = self.conn.prepare(sql)?;

        let interesting_fields = stmt.query_map(params![user_id], |row| {
            let field_str: String = row.get(1)?;
            Ok(UserInterestingField {
                user_id: row.get(0)?,
                field: field_str
                    .parse()  
                    .map_err(|e| rusqlite::Error::FromSqlConversionFailure(
                        2,
                        rusqlite::types::Type::Text,
                        Box::new(e),
                    ))?,
            })
        })?;

        interesting_fields.collect()
    }

    pub fn check_if_there_is_active_user_status(&self) -> Result<bool, rusqlite::Error> {
        let sql = "SELECT COUNT(*) FROM users WHERE status = 'active'";
    
        self.conn.query_row(sql, [], |row| {
            let count: i64 = row.get(0)?;
            Ok(count > 0)
        })
    }
    
    
}
