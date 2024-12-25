use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInterest {
    pub id: Option<i32>,
    pub user_id: i32,
    pub interest: String,
}
