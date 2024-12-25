use rusqlite::{Connection, Result};
use std::sync::{Arc, Mutex};
use std::path::Path;

pub mod db;
pub mod commands;

use db::run_migrations;
use commands::{create_user_command, check_if_there_is_active_user_status_command};

pub struct AppState {
    pub db_conn: Arc<Mutex<Connection>>,
}

impl AppState {
    pub fn new(db_path: &str) -> Result<Self> {
        if !Path::new(db_path).exists() {
            println!("Database at: {}", db_path);
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
    let db_path = "./app.db";

    let app_state = AppState::new(&db_path).expect("Falha ao inicializar o AppState");

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            create_user_command,
            check_if_there_is_active_user_status_command,

        ])
        .run(tauri::generate_context!())
        .expect("Erro ao rodar a aplicação Tauri");
}
