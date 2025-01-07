use serde::Serialize;
use rusqlite::{Connection, Error as RusqliteError};
use thiserror::Error;
use log::{error, info};
use crate::db::repositories::TagRepository;
use crate::db::models::Tag;
use crate::AppState;
use tauri::ipc::InvokeError;

#[derive(Debug, Error, Serialize)]
pub enum TagCommandError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Unexpected error: {0}")]
    UnexpectedError(String),
}

impl From<RusqliteError> for TagCommandError {
    fn from(err: RusqliteError) -> Self {
        TagCommandError::DatabaseError(err.to_string())
    }
}

pub struct TagCommands<'a> {
    repository: TagRepository<'a>
}

impl<'a> TagCommands<'a> {
    pub fn new(conn: &'a mut Connection) -> Self {
        let repository = TagRepository::new(conn);
        Self { repository }
    }

    pub fn create_tag_method(
        &mut self,
        title: String,
        color: String,
        icon: Option<String>,
    ) -> Result<String, TagCommandError> {
        info!("Starting the process of creating a new tag");

        if title.is_empty() {
            let msg = "No title provided".to_string();
            error!("{}", msg);
            return Err(TagCommandError::InvalidInput(msg))
        }

        if color.is_empty() {
            let msg = "No color provided".to_string();
            error!("{}", msg);
            return Err(TagCommandError::InvalidInput(msg))
        }

        let tag = Tag {
            id: None,
            title,
            color,
            icon
        };

        match self.repository.create_tag(&tag) {
            Ok(_tag_id) => {
                let sucess_msg = format!("tag created sucessfully");
                info!("{}", sucess_msg);
                Ok(sucess_msg)
            }
            Err(err) => {
                error!("Failed to create tag");
                Err(TagCommandError::DatabaseError(err.to_string()))
            }
        }
    }

    pub fn update_tag_method(
        &mut self,
        id: i32,
        title: Option<String>,
        color: Option<String>,
        icon: Option<Option<String>>,
    ) -> Result<String, TagCommandError> {
        info!("Starting the process of updating tag with ID {}", id);

        if title.is_none() && color.is_none() && icon.is_none() {
            let msg = "At least one field must be provided for update".to_string();
            error!("{}", msg);
            return Err(TagCommandError::InvalidInput(msg));
        }

        if let Some(new_title) =  title {
            if new_title.is_empty(){
                let msg = "Title cannot be empty".to_string();
                error!("{}", msg);
                return Err(TagCommandError::InvalidInput(msg));
            }
            match self.repository.update_title(id, &new_title){
                Ok(_) => info!("Updated title for tag with ID {}", id),
                Err(err) => return Err(TagCommandError::DatabaseError(err.to_string())),
            }
        }

        if let Some(new_color) = color {
            if new_color.is_empty() {
                let msg = "Color cannot be empty.".to_string();
                error!("{}", msg);
                return Err(TagCommandError::InvalidInput(msg));
            }
            match self.repository.update_color(id, &new_color) {
                Ok(_) => info!("Updated color for tag with ID {}", id),
                Err(err) => return Err(TagCommandError::DatabaseError(err.to_string())),
            }
        }

        if let Some(new_icon) = icon {
            match self.repository.update_icon(id, new_icon.as_deref()) {
                Ok(_) => info!("Updated icon for tag with ID {}", id),
                Err(err) => return Err(TagCommandError::DatabaseError(err.to_string())),
            }
        }
        
        Ok("Tag updated successfully.".to_string())
    }

    pub fn delete_tag_method(
        &mut self,
        id: i32
    ) -> Result<String, TagCommandError> {
        info!("Starting the process of deleting tag with ID {}", id);

        match self.repository.delete_tag(id) {
            Ok(_) => {
                info!("Tag with ID {} deleted successfully.", id);
                Ok(format!("Tag with ID {} deleted successfully.", id))
            }
            Err(err) => {
                error!("Failed to delete tag with ID {}: {}", id, err);
                Err(TagCommandError::DatabaseError(err.to_string()))
            }
        }
    }
    
    pub fn get_all_tags_method(
        &self,
    ) -> Result<Vec<Tag>, TagCommandError> {
        info!("Fetching all tags.");

        match self.repository.get_all_tags() {
            Ok(tags) => Ok(tags),
            Err(err) => {
                error!("Failed to fetch tags: {}", err);
                Err(TagCommandError::DatabaseError(err.to_string()))
            }
        }
    }
}

    #[tauri::command]
    pub fn create_tag_command(
        app_state: tauri::State<'_, AppState>,
        title: String,
        color: String,
        icon: Option<String>,
    ) -> Result<String, InvokeError> {
        let mut conn = app_state.db_conn.lock().unwrap();
        let mut tag_commands = TagCommands::new(&mut conn);

        match tag_commands.create_tag_method(title, color, icon) {
            Ok(result) => Ok(result),
            Err(err) => Err(InvokeError::from(err)),
        }
    }

#[tauri::command]
pub fn update_tag_command(
    app_state: tauri::State<'_, AppState>,
    id: i32,
    title: Option<String>,
    color: Option<String>,
    icon: Option<Option<String>>,
) -> Result<String, InvokeError> {
    let mut conn = app_state.db_conn.lock().unwrap();
    let mut tag_commands = TagCommands::new(&mut conn);

    match tag_commands.update_tag_method(id, title, color, icon) {
        Ok(result) => Ok(result),
        Err(err) => Err(InvokeError::from(err)),
    }
}

#[tauri::command]
pub fn delete_tag_command(
    app_state: tauri::State<'_, AppState>,
    id: i32,
) -> Result<String, InvokeError> {
    let mut conn = app_state.db_conn.lock().unwrap();
    let mut tag_commands = TagCommands::new(&mut conn);

    match tag_commands.delete_tag_method(id) {
        Ok(result) => Ok(result),
        Err(err) => Err(InvokeError::from(err)),
    }
}

#[tauri::command]
pub fn get_all_tags_command(
    app_state: tauri::State<'_, AppState>,
) -> Result<Vec<Tag>, InvokeError> {
    let mut conn = app_state.db_conn.lock().unwrap();
    let tag_commands = TagCommands::new(&mut conn);

    match tag_commands.get_all_tags_method() {
        Ok(tags) => Ok(tags),
        Err(err) => Err(InvokeError::from(err)),
    }
}