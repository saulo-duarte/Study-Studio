mod db;
pub use db::{create_user, initialize_database, check_user_and_redirect_command};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            create_user,
            initialize_database,
            check_user_and_redirect_command,
        ])
        .setup(|_app| {
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Erro ao executar a aplicação Tauri");
}
