use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub id: i64,
    pub title: String,
    pub color: String,
    pub description: Option<String>,
    pub created_at: String,
    pub icon: Option<String>,
}