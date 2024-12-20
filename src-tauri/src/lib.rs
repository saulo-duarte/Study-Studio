mod db;

pub use db::{initialize_database, insert_user, fetch_users};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            initialize_database,
            insert_user,
            fetch_users
        ])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
              }
            if let Err(e) = db::initialize_database() {
                eprintln!("Error initializing database: {}", e);
            }
            Ok(())
          
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
