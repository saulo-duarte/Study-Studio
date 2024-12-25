use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    pub status: UserStatus,
    pub created_at: Option<NaiveDateTime>,
    pub last_login: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum UserStatus {
    Active,
    Inactive,
}

impl UserStatus {
    pub fn from_str(status: &str) -> Option<Self> {
        match status {
            "active" => Some(Self::Active),
            "inactive" => Some(Self::Inactive),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
        }
    }
}
