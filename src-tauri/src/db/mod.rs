use crate::utils::path::get_database_path;
use rusqlite::{Connection, Result};

pub mod schemas {
    pub mod create_tables;
    pub mod user;
}

pub mod models;

pub use schemas::user::create_user;

#[tauri::command]
pub fn initialize_database() -> Result<String, String> {
    println!("Starting database initialization...");

    let db_path = get_database_path();
    let conn =
        Connection::open(&db_path).map_err(|e| format!("Failed to open the database: {}", e))?;

    println!("Enabling foreign keys...");
    conn.execute("PRAGMA foreign_keys = ON", [])
        .map_err(|e| format!("Failed to enable foreign keys: {}", e))?;

    println!("Creating tables...");

    let tables = [
        ("user", schemas::create_tables::CREATE_USER_TABLE),
        ("document", schemas::create_tables::CREATE_DOCUMENT_TABLE),
        ("task", schemas::create_tables::CREATE_TASK_TABLE),
        ("tag", schemas::create_tables::CREATE_TAG_TABLE),
        (
            "user_available_days",
            schemas::create_tables::CREATE_USER_AVAILABLE_DAYS_TABLE,
        ),
        (
            "user_interesting_fields",
            schemas::create_tables::CREATE_USER_INTERESTING_FIELDS_TABLE,
        ),
    ];

    for (table_name, create_table_query) in tables {
        println!("Creating table: {}", table_name);
        conn.execute(create_table_query, [])
            .map_err(|e| format!("Failed to create table '{}': {}", table_name, e))?;
        println!("Table {} created successfully", table_name);
    }

    println!("Database initialization completed!");
    Ok("Database initialized successfully".to_string())
}

pub fn check_user_and_redirect() -> Result<String, String> {
    let db_path = get_database_path();
    let conn =
        Connection::open(db_path).map_err(|e| format!("Failed to open the database: {}", e))?;

    let mut stmt = conn
        .prepare("SELECT COUNT(*) FROM user")
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;

    let user_count: i64 = stmt
        .query_row([], |row| row.get(0))
        .map_err(|e| format!("Error checking user count: {}", e))?;

    if user_count == 0 {
        return Ok("/onboarding".to_string());
    }

    Ok("/".to_string())
}

#[tauri::command]
pub fn check_user_and_redirect_command() -> Result<String, String> {
    check_user_and_redirect()
}
