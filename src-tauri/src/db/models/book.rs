use serde::{Deserialize, Serialize};
use crate::db::models::Tag;

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    pub id: i64,
    pub title: String,
    pub author: Option<String>,
    pub file_path: Option<String>,
    pub tags: Option<Vec<Tag>>
}