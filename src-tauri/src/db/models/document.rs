use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    pub id: i32,
    pub title: String,
    pub original_filename: String,
    pub stored_filename: String,
    pub file_path: String,
    pub file_size: String,
    pub mime_type: String,
    pub hash: String,
    pub page_count: i32,
    pub created_at: Option<DateTime<Utc>>,
    pub last_accessed: Option<DateTime<Utc>>,
    pub thumbnail_path: Option<String>,
    pub tags: Option<Vec<String>>,
}