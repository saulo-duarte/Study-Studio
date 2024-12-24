use rusqlite::Connection;
use crate::db::models::{FieldOfInteresting, User, UserAvailableDay, AvailableDay, UserInterestingField};
use crate::db::repositories::user_repository::UserRepository;
use chrono::{DateTime, Utc};
use crate::AppState;

pub struct UserCommands<'a> {
    repository: UserRepository<'a>,
}

impl<'a> UserCommands<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        let repository = UserRepository::new(conn);
        Self { repository }
    }

    pub fn create_user_internal(
        &self,
        name: &str,
        email: &str,
        status: &str,
        created_at: DateTime<Utc>,
        last_login: Option<DateTime<Utc>>,
        available_days: Vec<AvailableDay>,
        interesting_fields: Vec<FieldOfInteresting>,
    ) -> rusqlite::Result<User> {
        let user = User {
            id: 0,
            name: name.to_string(),
            email: email.to_string(),
            status: status.to_string(),
            created_at,
            last_login,
        };
        let created_user = self.repository.create(&user)?;

        for day in available_days {
            let user_available_day = UserAvailableDay {
                user_id: created_user.id,
                available_day: day,
            };
            self.repository.create_user_available_day(&user_available_day)?;
        }

        for field in interesting_fields {
            let user_interesting_field = UserInterestingField {
                user_id: created_user.id,
                field,
            };
            self.repository.create_user_interesting_field(&user_interesting_field)?;
        }

        Ok(created_user)
    }

    pub fn find_user_by_id(&self, user_id: i32) -> rusqlite::Result<User> {
        self.repository.find_by_id(user_id)
    }

    pub fn list_all_users(&self) -> rusqlite::Result<Vec<User>> {
        self.repository.find_all()
    }

    pub fn list_available_days_for_user(&self, user_id: i32) -> rusqlite::Result<Vec<UserAvailableDay>> {
        self.repository.find_user_available_days_by_user_id(user_id)
    }

    pub fn list_interesting_fields_for_user(
        &self,
        user_id: i32,
    ) -> rusqlite::Result<Vec<UserInterestingField>> {
        self.repository.find_user_interesting_fields_by_user_id(user_id)
    }

    pub fn check_if_there_is_active_user_status(
        &self,
    ) -> rusqlite::Result<bool> {
        self.repository.check_if_there_is_active_user_status()
    }
}


#[tauri::command]
pub fn create_user(
    app_state: tauri::State<AppState>,
    name: String,
    email: String,
    status: String,
    available_days: Vec<AvailableDay>,
    interesting_fields: Vec<FieldOfInteresting>,
) -> Result<User, String> {
    let conn = app_state.db_conn.lock().unwrap();
    let commands = UserCommands::new(&conn);

    let created_at = Utc::now();
    let last_login = None;

    match commands.create_user_internal(&name, &email, &status, created_at, last_login, available_days, interesting_fields) {
        Ok(user) => Ok(user),
        Err(e) => Err(format!("Erro ao criar usuário: {}", e)),
    }
}


#[tauri::command]
pub fn find_user_by_id(app_state: tauri::State<AppState>, user_id: i32) -> Result<User, String> {
    let conn = app_state.db_conn.lock().unwrap();
    let commands = UserCommands::new(&conn);

    commands
        .find_user_by_id(user_id)
        .map_err(|err| err.to_string())
}

#[tauri::command]
pub fn list_all_users(app_state: tauri::State<AppState>) -> Result<Vec<User>, String> {
    let conn = app_state.db_conn.lock().unwrap();
    let commands = UserCommands::new(&conn);

    commands
        .list_all_users()
        .map_err(|err| err.to_string())
}


#[tauri::command]
pub fn check_if_there_is_active_user_status(
    app_state: tauri::State<AppState>,
) -> Result<bool, String> {
    let conn = app_state.db_conn.lock().unwrap();
    
    let commands = UserCommands::new(&conn);

    commands
        .check_if_there_is_active_user_status()
        .map_err(|err| err.to_string())
}
