use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::fmt;
use std::error::Error;

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

impl FromStr for AvailableDay {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "monday" => Ok(AvailableDay::Monday),
            "tuesday" => Ok(AvailableDay::Tuesday),
            "wednesday" => Ok(AvailableDay::Wednesday),
            "thursday" => Ok(AvailableDay::Thursday),
            "friday" => Ok(AvailableDay::Friday),
            "saturday" => Ok(AvailableDay::Saturday),
            "sunday" => Ok(AvailableDay::Sunday),
            _ => Err(format!("Invalid day: {}", s)),
        }
    }
}

impl ToString for AvailableDay {
    fn to_string(&self) -> String {
        match self {
            AvailableDay::Monday => "Monday".to_string(),
            AvailableDay::Tuesday => "Tuesday".to_string(),
            AvailableDay::Wednesday => "Wednesday".to_string(),
            AvailableDay::Thursday => "Thursday".to_string(),
            AvailableDay::Friday => "Friday".to_string(),
            AvailableDay::Saturday => "Saturday".to_string(),
            AvailableDay::Sunday => "Sunday".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAvailableDay {
    pub user_id: i32,
    pub available_day: AvailableDay,
}

impl UserAvailableDay {
    pub fn available_day_str(&self) -> String {
        self.available_day.to_string()
    }
}

#[derive(Debug)]
pub enum AvailableDayParseError {
    InvalidFormat(String),
}

impl fmt::Display for AvailableDayParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AvailableDayParseError::InvalidFormat(s) => write!(f, "Invalid format: {}", s),
        }
    }
}

impl Error for AvailableDayParseError {}

