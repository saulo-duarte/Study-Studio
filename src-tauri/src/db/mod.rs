use rusqlite::{Connection, Result};

pub mod schemas {
    pub mod create_tables;
    pub mod user;
}

pub mod models;

pub use schemas::user::create_user;

fn table_exists(conn: &Connection, table_name: &str) -> Result<bool, String> {
    let query = format!(
        "SELECT name FROM sqlite_master WHERE type='table' AND name='{}';",
        table_name
    );

    let mut stmt = conn.prepare(&query)
        .map_err(|e| e.to_string())?;

    let mut rows = stmt.query([])
        .map_err(|e| e.to_string())?;

    Ok(rows.next().map(|_| true).unwrap_or(false))
}

fn column_exists(conn: &Connection, table_name: &str, column_name: &str) -> Result<bool, String> {
    let query = format!(
        "PRAGMA table_info({});",
        table_name
    );

    let mut stmt = conn.prepare(&query)
        .map_err(|e| e.to_string())?;

    let rows = stmt.query_map([], |row| {
        let name: String = row.get(1)?;
        Ok(name)
    }).map_err(|e| e.to_string())?;

    for row in rows {
        if let Ok(column) = row {
            if column == column_name {
                return Ok(true);
            }
        }
    }

    Ok(false)
}

#[tauri::command]
pub fn initialize_database() -> Result<String, String> {
    let conn = Connection::open("database.db")
        .map_err(|e| e.to_string())?;

    match table_exists(&conn, "user") {
        Ok(false) => {
            conn.execute(schemas::create_tables::CREATE_USER_TABLE, [])
                .map_err(|e| format!("Failed to create 'user' table: {}", e))?;
        }
        Ok(true) => println!("'user' table already exists."),
        Err(e) => return Err(format!("Error checking 'user' table: {}", e)),
    }

    match column_exists(&conn, "user", "status") {
        Ok(false) => {
            conn.execute(
                "ALTER TABLE user ADD COLUMN status TEXT NOT NULL DEFAULT 'ativo'",
                [],
            )
            .map_err(|e| format!("Failed to add 'status' column: {}", e))?;
            println!("Added 'status' column to 'user' table.");
        }
        Ok(true) => println!("'status' column already exists in 'user' table."),
        Err(e) => return Err(format!("Error checking 'status' column: {}", e)),
    }

    match table_exists(&conn, "document") {
        Ok(false) => {
            conn.execute(schemas::create_tables::CREATE_DOCUMENT_TABLE, [])
                .map_err(|e| format!("Failed to create 'document' table: {}", e))?;
        }
        Ok(true) => println!("'document' table already exists."),
        Err(e) => return Err(format!("Error checking 'document' table: {}", e)),
    }

    match table_exists(&conn, "task") {
        Ok(false) => {
            conn.execute(schemas::create_tables::CREATE_TASK_TABLE, [])
                .map_err(|e| format!("Failed to create 'task' table: {}", e))?;
        }
        Ok(true) => println!("'task' table already exists."),
        Err(e) => return Err(format!("Error checking 'task' table: {}", e)),
    }

    match table_exists(&conn, "tag") {
        Ok(false) => {
            conn.execute(schemas::create_tables::CREATE_TAG_TABLE, [])
                .map_err(|e| format!("Failed to create 'tag' table: {}", e))?;
        }
        Ok(true) => println!("'tag' table already exists."),
        Err(e) => return Err(format!("Error checking 'tag' table: {}", e)),
    }

    Ok("Database initialized successfully".to_string())
    
}

pub fn check_user_and_redirect() -> Result<String, String> {
    let conn = Connection::open("database.db").map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT COUNT(*) FROM user")
        .map_err(|e| e.to_string())?;

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