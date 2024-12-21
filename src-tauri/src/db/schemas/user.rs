use rusqlite::{Connection, Result};
use chrono::Utc;
use crate::db::models::{FieldOfInteresting, AvailableDay};

#[tauri::command]
pub fn create_user(
    name: String,
    email: Option<String>,
    interesting_fields: Vec<FieldOfInteresting>,
    available_days: Vec<AvailableDay>,
    status: Option<String>
) -> Result<String, String> {
    let conn = Connection::open("database.db")
        .map_err(|e| e.to_string())?;

    let created_at = Utc::now().to_rfc3339();
    let last_login = created_at.clone();
    let user_status = status.unwrap_or_else(||"active".to_string()); 

    let mut stmt = conn.prepare(
        "INSERT INTO user 
            (name, email, created_at, last_login, interesting_fields, available_days, status) 
         VALUES 
            (?1, ?2, ?3, ?4, ?5, ?6)",
    )
    .map_err(|e| e.to_string())?;

    let interesting_fields_str = serde_json::to_string(&interesting_fields)
        .map_err(|e| e.to_string())?;
    let available_days_str = serde_json::to_string(&available_days)
        .map_err(|e| e.to_string())?;

    stmt.execute(rusqlite::params![
        name,
        email,
        created_at,
        last_login,
        interesting_fields_str,
        available_days_str,
        user_status
    ])
    .map_err(|e| format!("Failed to insert user: {}", e))?;

    Ok("User created successfully".to_string())
}

                                                                                                        