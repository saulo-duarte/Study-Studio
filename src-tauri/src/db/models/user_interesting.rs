use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum FieldOfInteresting {
    Frontend,
    Backend,
    Mobile,
    DataScience,
    DataEngineering,
    DevOps,
}

impl fmt::Display for FieldOfInteresting {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_str = match self {
            FieldOfInteresting::Frontend => "frontend",
            FieldOfInteresting::Backend => "backend",
            FieldOfInteresting::Mobile => "mobile",
            FieldOfInteresting::DataScience => "data-science",
            FieldOfInteresting::DataEngineering => "data-engineering",
            FieldOfInteresting::DevOps => "devops",
        };
        write!(f, "{}", display_str)
    }
}

impl FromStr for FieldOfInteresting {
    type Err = FieldOfInterestingParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "frontend" => Ok(FieldOfInteresting::Frontend),
            "backend" => Ok(FieldOfInteresting::Backend),
            "mobile" => Ok(FieldOfInteresting::Mobile),
            "data-science" => Ok(FieldOfInteresting::DataScience),
            "data-engineering" => Ok(FieldOfInteresting::DataEngineering),
            "devops" => Ok(FieldOfInteresting::DevOps),
            _ => Err(FieldOfInterestingParseError::new(&format!("Invalid field of interest: {}", s))),
        }
    }
}



#[derive(Debug, Serialize, Deserialize)]
pub struct UserInterestingField {
    pub user_id: i32,
    pub field: FieldOfInteresting,
}

#[derive(Debug)]
pub struct FieldOfInterestingParseError {
    pub details: String,
}

impl FieldOfInterestingParseError {
    fn new(msg: &str) -> Self {
        FieldOfInterestingParseError {
            details: msg.to_string(),
        }
    }
}

impl std::fmt::Display for FieldOfInterestingParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "FieldOfInterestingParseError: {}", self.details)
    }
}

impl std::error::Error for FieldOfInterestingParseError {}