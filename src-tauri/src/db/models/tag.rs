use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub id: Option<i32>,
    pub title: String,
    pub color: String,
    pub icon: Option<String>,
}