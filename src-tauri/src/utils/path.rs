use std::path::PathBuf;

pub fn get_database_path() -> PathBuf {
    let mut db_path = std::env::current_dir().expect("Failed to get the current directory");
    println!("Database path: {:?}", db_path);
    db_path.push("database.db");
    db_path
    
}
