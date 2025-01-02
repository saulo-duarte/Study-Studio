use serde::Serialize;
use rusqlite::{Connection, Error as RusqliteError};
use thiserror::Error;
use log::{error, info};
use std::sync::MutexGuard;
use crate::db::repositories::{self, BookRepository};
use crate::db::models::{book::Book, tag::Tag};
use crate::AppState;
use tauri::ipc::InvokeError;

#[derive(Debug, Error, Serialize)]
pub enum BookCommandError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Unexpected error: {0}")]
    UnexpectedError(String),
}

impl From<RusqliteError> for BookCommandError {
    fn from(err: RusqliteError) -> Self {
        BookCommandError::DatabaseError(err.to_string())
    }
}

pub struct BookCommands<'a> {
    repository: BookRepository<'a>,
}

impl<'a> BookCommands<'a> {
    pub fn new(conn: &'a mut Connection) -> Self {
        let repository = BookRepository::new(conn);
        Self { repository }
    }
    
    pub fn insert_new_book_method(
        &mut self,
        title: String,
        author: Option<String>,
        file_path: Option<String>,
    ) -> Result<String, BookCommandError> {
        info!("Starting the process of inserting a new book");
        
        if title.is_empty() {
            let msg = "You must provide a title".to_string();
            error!("{}", msg);
            return Err(BookCommandError::InvalidInput(msg));
        }

        let book = Book {
            id: 0,
            title,
            author,
            file_path,
            tags: None,
        };

        match self.repository.create_book(&book) {
            Ok(book_id) => {
                let success_msg = format!("Book inserted successfully with ID {}", book_id);
                info!("{}", success_msg);
                Ok(success_msg)
            }
            Err(err) => {
                error!("Failed to insert book");
                Err(BookCommandError::DatabaseError(err.to_string()))
            }
        }
    }

    pub fn update_book_method(
        &mut self,
        id: i64,
        title: Option<String>,
        author: Option<String>,
        file_path: Option<String>,
    ) -> Result<String, BookCommandError> {
        info!("Starting the process of updating book with ID {}", id);

        if title.as_ref().map_or(false, |t| t.is_empty()) {
            let msg = "Title cannot be empty".to_string();
            error!("{}", msg);
            return Err(BookCommandError::InvalidInput(msg));
        }

        let existing_book = match self.repository.get_book_by_id(id)? {
            Some(book) => book,
            None => {
                let msg = format!("Book with ID {} not found", id);
                error!("{}", msg);
                return Err(BookCommandError::InvalidInput(msg));
            }
        };

        let book = Book {
            id,
            title: title.unwrap_or(existing_book.title),
            author: author.or(existing_book.author),
            file_path: file_path.or(existing_book.file_path),
            tags: existing_book.tags,
        };

        match self.repository.update_book(&book) {
            Ok(_) => {
                let success_msg = format!("Book with ID {} updated successfully", id);
                info!("{}", success_msg);
                Ok(success_msg)
            }
            Err(err) => {
                error!("Failed to update book with ID {}: {}", id, err);
                Err(BookCommandError::DatabaseError(err.to_string()))
            }
        }
    }

    pub fn delete_book_method(
        &mut self,
        id: i64,
    ) -> Result<String, BookCommandError> {
        info!("Starting the process of deleting book with ID {}", id);

        match self.repository.delete_book(id) {
            Ok(_) => {
                let success_msg = format!("Book with ID {} deleted successfully", id);
                info!("{}", success_msg);
                Ok(success_msg)
            }
            Err(err) => {
                error!("Failed to delete book with ID {}: {}", id, err);
                Err(BookCommandError::DatabaseError(err.to_string()))
            }
        }
    }

    pub fn get_all_books_method(&self) -> Result<Vec<Book>, BookCommandError> {
        info!("Fetching all books");
        
        match self.repository.get_all_books() {
            Ok(books) => {
                info!("Successfully fetched {} books", books.len());
                Ok(books)
            }
            Err(err) => {
                error!("Failed to fetch books: {}", err);
                Err(BookCommandError::DatabaseError(err.to_string()))
            }
        }
    }

    pub fn add_tags_to_book_method(
        &mut self,
        book_id: i64,
        tags: Vec<Tag>,
    ) -> Result<String, BookCommandError> {
        info!("Starting process of adding tags to book {}", book_id);

        if tags.is_empty() {
            let msg = "No tags provided".to_string();
            error!("{}", msg);
            return Err(BookCommandError::InvalidInput(msg));
        }

        match self.repository.add_tags_to_book(book_id, tags) {
            Ok(_) => {
                let success_msg = format!("Tags added successfully to book {}", book_id);
                info!("{}", success_msg);
                Ok(success_msg)
            }
            Err(err) => {
                error!("Failed to add tags to book {}: {}", book_id, err);
                Err(BookCommandError::DatabaseError(err.to_string()))
            }
        }
    }

    pub fn remove_tags_from_book_method(
        &mut self,
        book_id: i64,
        tags: Vec<Tag>,
    ) -> Result<String, BookCommandError> {
        info!("Starting process of removing tags from book {}", book_id);

        if tags.is_empty() {
            let msg = "No tags provided".to_string();
            error!("{}", msg);
            return Err(BookCommandError::InvalidInput(msg));
        }

        match self.repository.remove_tags_from_book(book_id, tags) {
            Ok(_) => {
                let success_msg = format!("Tags removed successfully from book {}", book_id);
                info!("{}", success_msg);
                Ok(success_msg)
            }
            Err(err) => {
                error!("Failed to remove tags from book {}: {}", book_id, err);
                Err(BookCommandError::DatabaseError(err.to_string()))
            }
        }
    }
}


// Tauri commands
#[tauri::command]
pub fn insert_new_book_command(
    app_state: tauri::State<'_, AppState>,
    title: String,
    author: Option<String>,
    file_path: Option<String>,
) -> Result<String, InvokeError> {
    let mut conn = app_state.db_conn.lock().unwrap();
    let mut book_commands = BookCommands::new(&mut conn); 

    match book_commands.insert_new_book_method(title, author, file_path) {
        Ok(result) => Ok(result),
        Err(err) => Err(InvokeError::from(err)),
    }
}



#[tauri::command]
pub fn update_book_command(
    app_state: tauri::State<'_, AppState>,
    id: i64,
    title: Option<String>,
    author: Option<String>,
    file_path: Option<String>,
) -> Result<String, InvokeError> {
    let mut conn = app_state.db_conn.lock().unwrap();
    let mut book_commands = BookCommands::new(&mut conn); 
    
    match book_commands.update_book_method(id, title, author, file_path) {
        Ok(result) => Ok(result),
        Err(err) => Err(InvokeError::from(err))
    }
}

#[tauri::command]
pub fn delete_book_command(
    app_state: tauri::State<'_, AppState>,
    id: i64,
) -> Result<String, InvokeError> {
    let mut conn = app_state.db_conn.lock().unwrap();
    let mut book_commands = BookCommands::new(&mut conn); 
    
    match book_commands.delete_book_method(id) {
        Ok(result) => Ok(result),
        Err(err) => Err(InvokeError::from(err))
    }
}

#[tauri::command]
pub fn get_all_books_command(
    app_state: tauri::State<'_, AppState>,
) -> Result<Vec<Book>, InvokeError> {
    let mut conn = app_state.db_conn.lock().unwrap();
    let mut book_commands = BookCommands::new(&mut conn); 
    
    match book_commands.get_all_books_method() {
        Ok(books) => Ok(books),
        Err(err) => Err(InvokeError::from(err))
    }
}

#[tauri::command]
pub fn add_tags_to_book_command(
    app_state: tauri::State<'_, AppState>,
    book_id: i64,
    tags: Vec<Tag>,
) -> Result<String, InvokeError> {
    let mut conn = app_state.db_conn.lock().unwrap();
    let mut book_commands = BookCommands::new(&mut conn); 
    
    match book_commands.add_tags_to_book_method(book_id, tags) {
        Ok(result) => Ok(result),
        Err(err) => Err(InvokeError::from(err))
    }
}

#[tauri::command]
pub fn remove_tags_from_book_command(
    app_state: tauri::State<'_, AppState>,
    book_id: i64,
    tags: Vec<Tag>,
) -> Result<String, InvokeError> {
    let mut conn = app_state.db_conn.lock().unwrap();
    let mut book_commands = BookCommands::new(&mut conn); 
    
    match book_commands.remove_tags_from_book_method(book_id, tags) {
        Ok(result) => Ok(result),
        Err(err) => Err(InvokeError::from(err))
    }
}