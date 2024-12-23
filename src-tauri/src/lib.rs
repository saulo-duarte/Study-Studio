mod db;
mod utils;
use db::{create_user, initialize_database, check_user_and_redirect_command};
use utils::path::get_database_path;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            create_user,
            initialize_database,
            check_user_and_redirect_command,
        ])
        .setup(|_app| {
            let db_path = get_database_path();
            println!("Database path: {:?}", db_path);
            
            // Ensure the database file and directory exist
            std::fs::create_dir_all(db_path.parent().unwrap())
                .expect("Failed to create database directory");
            
            // Initialize the database during setup
            match initialize_database() {
                Ok(_) => println!("Database initialized successfully during setup"),
                Err(e) => eprintln!("Error initializing database during setup: {}", e)
            }
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Failed to run Tauri application");
}