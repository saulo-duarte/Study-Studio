use serde::Serialize;
use rusqlite::{Connection, Error as RusqliteError};
use thiserror::Error;
use log::{error, info};
use crate::db::repositories::UserRepository;
use crate::db::models::{
    user::{UserStatus, User},
    user_available_day::DayOfWeek,
};
use crate::AppState;
use tauri::ipc::InvokeError;

#[derive(Debug, Error, Serialize)]
pub enum UserCommandError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Unexpected error: {0}")]
    UnexpectedError(String),
}

impl From<RusqliteError> for UserCommandError {
    fn from(err: RusqliteError) -> Self {
        UserCommandError::DatabaseError(err.to_string())
    }
}

pub struct UserCommands<'a> {
    repository: UserRepository<'a>,
}

impl<'a> UserCommands<'a> {
    pub fn new(conn: &'a mut Connection) -> Self {
        let repository = UserRepository::new(conn);
        Self { repository }
    }

    pub fn create_user_method(
        &mut self,
        name: String,
        email: String,
        status: String,
        available_days: Vec<String>,
        interests: Vec<String>,
    ) -> Result<String, UserCommandError> {
        info!("Starting the process of creating a new user");

        // Parse user status
        let status = UserStatus::from_str(&status).ok_or_else(|| {
            let msg = format!("Invalid status provided: {}", status);
            error!("{}", msg);
            UserCommandError::InvalidInput(msg)
        })?;

        // Parse available days
        let available_days = available_days
            .iter()
            .filter_map(|day| {
                DayOfWeek::from_str(day).or_else(|| {
                    error!("Invalid day of week provided: {}", day);
                    None
                })
            })
            .collect::<Vec<DayOfWeek>>();

        if available_days.is_empty() {
            let msg = "No valid days of the week provided".to_string();
            error!("{}", msg);
            return Err(UserCommandError::InvalidInput(msg));
        }

        // Create user struct
        let user = User {
            id: None,
            name,
            email,
            status,
            created_at: None,
            last_login: None,
        };

        // Call repository to insert user
        match self.repository.create_user(&user, available_days, interests) {
            Ok(user_id) => {
                let success_msg = format!("User created successfully with ID: {}", user_id);
                info!("{}", success_msg);
                Ok(success_msg)
            }
            Err(err) => {
                error!("Failed to create user: {}", err);
                Err(UserCommandError::DatabaseError(err.to_string()))
            }
        }
    }

    pub fn check_if_there_is_active_user_status_method(
        &mut self) -> Result<bool, UserCommandError> {
            info!("Checking if there is an active user");

            // Call repository to check if there is an active user

            match self.repository.check_if_there_is_active_user_status() {
                Ok(result) => {
                    if result {
                        info!("There is at least one active user.");
                    } else {
                        info!("No active users found.");
                    }
                    Ok(result)
                }
                Err(err) => {
                    error!("Failed to check active user status {}", err);
                    Err(UserCommandError::DatabaseError(err.to_string()))
                }
            }
        }
}

#[tauri::command]
pub fn create_user_command(
    app_state: tauri::State<'_, AppState>,
    name: String,
    email: String,
    status: String,
    available_days: Vec<String>,
    interests: Vec<String>,
) -> Result<String, InvokeError> {
    // Lock the database connection to ensure thread safety
    let mut conn = app_state.db_conn.lock().unwrap();
    
    let mut user_commands = UserCommands::new(&mut conn);

    match user_commands.create_user_method(name, email, status, available_days, interests) {
        Ok(result) => Ok(result),
        Err(err) => Err(InvokeError::from(err)),
    }
}

#[tauri::command]
pub fn check_if_there_is_active_user_status_command(
    app_state: tauri::State<'_, AppState>,
) -> Result<bool, InvokeError> {
    
    let mut conn = app_state.db_conn.lock().unwrap();

    let mut user_commands = UserCommands::new(&mut conn);

    // Call the function and convert any errors to InvokeError

    match user_commands.check_if_there_is_active_user_status_method(){
        Ok(result) =>  Ok(result),
        Err(err) => Err(InvokeError::from(err)),
    }
}
