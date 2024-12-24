use rusqlite::{Connection, Result};
use std::sync::{Arc, Mutex};
use std::path::Path;

pub mod db;
pub mod commands;

use db::run_migrations;
use commands::user_commands;

pub struct AppState {
    pub db_conn: Arc<Mutex<Connection>>, 
}

impl AppState {
    pub fn new(db_path: &str) -> Result<Self> {
        if !Path::new(db_path).exists() {
            println!("Banco de dados será criado em: {}", db_path);
        }

        let conn = Connection::open(db_path)?;

        run_migrations(&conn)?;

        Ok(Self {
            db_conn: Arc::new(Mutex::new(conn)),
        })
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let db_path = "./app.db";

    let app_state = AppState::new(&db_path).expect("Falha ao inicializar o AppState");

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            user_commands::create_user, 
            user_commands::list_all_users,
            user_commands::check_if_there_is_active_user_status,
        ])
        .run(tauri::generate_context!())
        .expect("Erro ao rodar a aplicação Tauri");
}
