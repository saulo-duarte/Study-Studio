use rusqlite::{Connection, Result};
use chrono::Utc;
use crate::db::models::{AvailableDay, FieldOfInteresting, UserAvailableDay, UserInterestingField};
use crate::utils::path::get_database_path;

#[tauri::command]
pub fn create_user(
    name: String,
    email: Option<String>,
    interesting_fields: String, // This will still come as a string, so we can split it later
    available_days: Vec<AvailableDay>,
    status: Option<String>
) -> Result<String, String> {
    let db_path = get_database_path();
    let conn = Connection::open(db_path)
        .map_err(|e| format!("Failed to connect to database: {}", e))?;

    let created_at = Utc::now().to_rfc3339();
    let last_login = created_at.clone();
    let user_status = status.unwrap_or_else(|| "active".to_string());

    // Inserir o usuário na tabela 'user'
    let mut stmt = conn.prepare(
        "INSERT INTO user (name, email, created_at, last_login, status)
         VALUES (?1, ?2, ?3, ?4, ?5)"
    ).map_err(|e| format!("Failed to prepare statement: {}", e))?;

    stmt.execute(rusqlite::params![
        name,
        email,
        created_at,
        last_login,
        user_status
    ]).map_err(|e| format!("Failed to insert user: {}", e))?;

    let user_id = conn.last_insert_rowid();

    // Inserir os campos de interesse (interesting_fields)
    let interesting_fields_vec: Vec<FieldOfInteresting> = interesting_fields
        .split(',')
        .filter_map(|field| match field.trim().to_lowercase().as_str() {
            "frontend" => Some(FieldOfInteresting::Frontend),
            "backend" => Some(FieldOfInteresting::Backend),
            "mobile" => Some(FieldOfInteresting::Mobile),
            "data-science" => Some(FieldOfInteresting::DataScience),
            "data-engineering" => Some(FieldOfInteresting::DataEngineering),
            "devops" => Some(FieldOfInteresting::DevOps),
            _ => None,
        })
        .collect();

    for field in interesting_fields_vec {
        let user_interesting_field = UserInterestingField {
            user_id: user_id as i32,
            field,
        };

        let mut stmt = conn.prepare(
            "INSERT INTO user_interesting_fields (user_id, field)
             VALUES (?1, ?2)"
        ).map_err(|e| format!("Failed to prepare statement for interesting_fields: {}", e))?;

        stmt.execute(rusqlite::params![
            user_interesting_field.user_id,
            serde_json::to_string(&user_interesting_field.field).unwrap()
        ]).map_err(|e| format!("Failed to insert user_interesting_field: {}", e))?;
    }

    // Inserir os dias disponíveis (available_days)
    for day in available_days {
        let user_available_day = UserAvailableDay {
            user_id: user_id as i32,
            available_day: day,
        };

        let mut stmt = conn.prepare(
            "INSERT INTO user_available_days (user_id, available_day)
             VALUES (?1, ?2)"
        ).map_err(|e| format!("Failed to prepare statement for available_days: {}", e))?;

        stmt.execute(rusqlite::params![
            user_available_day.user_id,
            serde_json::to_string(&user_available_day.available_day).unwrap()
        ]).map_err(|e| format!("Failed to insert user_available_day: {}", e))?;
    }

    Ok("User created successfully".to_string())
}
