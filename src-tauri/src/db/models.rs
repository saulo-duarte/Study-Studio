use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize)]
pub enum FieldOfInteresting {
    Frontend,
    Backend,
    Mobile,
    DataScience,
    DataEngineering,
    DevOps,
}

impl<'de> Deserialize<'de> for FieldOfInteresting {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "frontend" => Ok(FieldOfInteresting::Frontend),
            "backend" => Ok(FieldOfInteresting::Backend),
            "mobile" => Ok(FieldOfInteresting::Mobile),
            "data-science" => Ok(FieldOfInteresting::DataScience),
            "data-engineering" => Ok(FieldOfInteresting::DataEngineering),
            "devops" => Ok(FieldOfInteresting::DevOps),
            _ => Err(serde::de::Error::unknown_variant(&s, &["frontend", "backend", "mobile", "data-science", "data-engineering", "devops"])),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AvailableDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

// Struct for User entity
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: Option<String>,
    pub created_at: String,
    pub last_login: Option<String>,
    pub status: String,
}

// Struct for User-AvailableDay relation
#[derive(Debug, Serialize, Deserialize)]
pub struct UserAvailableDay {
    pub user_id: i32,
    pub available_day: AvailableDay,
}

// Struct for User-FieldOfInterest relation
#[derive(Debug, Serialize, Deserialize)]
pub struct UserInterestingField {
    pub user_id: i32,
    pub field: FieldOfInteresting,
}

// Struct for Document entity
#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    pub id: i32,
    pub title: String,
    pub original_filename: String,
    pub stored_filename: String,
    pub file_path: String,
    pub file_size: i64,
    pub mime_type: String,
    pub hash: String,
    pub page_count: i32,
    pub created_at: String,
    pub last_accessed: Option<String>,
    pub thumbnail_path: Option<String>,
    pub tags: Vec<Tag>,
}

// Struct for Task entity
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

// Struct for Tag entity
#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub id: i64,
    pub title: String,
    pub color: String,
    pub description: Option<String>,
    pub created_at: String,
    pub icon: Option<String>,
}
