use serde::{Deserialize, Serialize};
use crate::db::models::Tag;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: String,
    pub due_date: Option<String>,
    pub completed_at: Option<String>,
    pub document_id: Option<i64>,
    pub tags: Vec<Tag>,
}
