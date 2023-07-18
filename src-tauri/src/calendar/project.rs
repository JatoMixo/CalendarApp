use serde::{Serialize, Deserialize};
use serde_json::{Value, json};
use crate::calendar::date::Date;
use crate::json::error::FileError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub color: String,
    pub description: String,
    pub start_date: Date,
    pub final_date: Date,
}

impl Project {
    pub fn to_json(&self) -> Value {
        json!({
            "name": self.name,
            "color": self.color,
            "description": self.description,
            "start_date": self.start_date,
            "final_date": self.final_date,
        })
    }

    pub fn from_json(value: Value) -> Result<Project, FileError> {
        match serde_json::from_value(value) {
            Ok(project) => Ok(project),
            Err(_) => Err(FileError::ParseError),
        }
    }
}