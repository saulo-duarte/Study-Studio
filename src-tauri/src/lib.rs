use rusqlite::{Connection, Result};
use std::sync::{Arc, Mutex};
use std::fs;
use std::path::PathBuf;
use std::io;

pub mod db;
pub mod commands;

use db::run_migrations;
use commands::{create_user_command, check_if_there_is_active_user_status_command};

pub struct AppState {
    pub db_conn: Arc<Mutex<Connection>>,
}

impl AppState {
    pub fn new(db_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let db_path = PathBuf::from(db_path);

        // Certifique-se de que o diretório exista
        if let Some(parent) = db_path.parent() {
            // Usando map_err para mapear o erro de I/O para um erro genérico
            fs::create_dir_all(parent).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        }

        if !db_path.exists() {
            println!("Database at: {:?}", db_path);
        }

        let conn = Connection::open(db_path)?;

        run_migrations(&conn)?;

        Ok(Self {
            db_conn: Arc::new(Mutex::new(conn)),
        })
    }

    pub fn db_conn(&self) -> Arc<Mutex<Connection>> {
        Arc::clone(&self.db_conn)
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Determinar o caminho da database
    let mut db_path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    db_path.push("study-studio");
    db_path.push("app.db");

    let db_path_str = db_path.to_string_lossy().to_string();

    let app_state = AppState::new(&db_path_str).expect("Falha ao inicializar o AppState");

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            create_user_command,
            check_if_there_is_active_user_status_command,
        ])
        .run(tauri::generate_context!())
        .expect("Erro ao rodar a aplicação Tauri");
}
