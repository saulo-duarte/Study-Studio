use rusqlite::Connection;
use serde::{Serialize, Deserialize};
use std::result::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: Option<String>,
    pub created_at: String,
    pub last_login: Option<String>,
    pub preferences: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    pub id: i32,
    pub title: String,
    pub original_filename: String,
    pub stored_filename: String,
    pub file_path: String,
    pub file_size:i64;
    pub mime_type: String,
    pub hash: String,
    pub page_count: i32,
    pub created_at: String,
    pub last_accessed: Option<String>,
    pub thumbnail_path: Option<String>
    pub tags: Vec<Tag>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: String,
    pub due_date: Option<String>,
    pub completed_at: String,
    pub document_id: Option<i64>
    pub tags: Vec<Tag>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub id: i64,
    pub title: String,
    pub color: String,
    pub description: Option<String>,
    pub created_at: String,
    pub icon: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct