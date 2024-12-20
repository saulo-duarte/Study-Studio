use rusqlite::{Connection, Result};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub status: String,
    pub name: String,
}

#[tauri::command]
pub fn initialize_database() -> Result<String, String> {
    let conn = Connection::open("database.db").map_err(|e| e.to_string())?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS user (
            status TEXT NOT NULL,
            name TEXT NOT NULL
        )",
        [],
    )
    .map_err(|e| e.to_string())?;

    Ok("Database initialized".to_string())
}

#[tauri::command]
pub fn insert_user(status: String, name: String) -> Result<String, String> {
    let conn = Connection::open("database.db").map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO user (status, name) VALUES (?1, ?2)",
        [status, name],
    )
    .map_err(|e| e.to_string())?;
    Ok("User inserted successfully!".to_string())
}

#[tauri::command]
pub fn fetch_users() -> Result<Vec<User>, String> {
    let conn = Connection::open("database.db").map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT status, name FROM user").map_err(|e| e.to_string())?;
    let user_iter = stmt
        .query_map([], |row| {
            Ok(User {
                status: row.get(0)?,
                name: row.get(1)?,
            })
        })
        .map_err(|e| e.to_string())?;
    let users: Vec<_> = user_iter.filter_map(Result::ok).collect();
    Ok(users)
}
