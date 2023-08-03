use serde::{Serialize, Deserialize};
use serde_json::{Value, json};
use crate::json::error::FileError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Date {
    pub day: u8,
    pub month: u8,
    pub year: u16,
}

impl Date {
    pub fn month_to_string(&self) -> Option<String> {
        const MONTHS: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

        if self.month as usize >= MONTHS.len() {
            return None;
        }

        Some(MONTHS[self.month as usize].to_string())
    }

    pub fn to_json(&self) -> Value {
        json!({
            "day": self.day,
            "month": self.month,
            "year": self.year,
        })
    }

    pub fn from_json(value: Value) -> Result<Date, FileError> {
        match serde_json::from_value(value) {
            Ok(date) => Ok(date),
            Err(_) => Err(FileError::ParseError),
        }
    }

    pub fn to_int(&self) -> u64 {
        (self.year as u64) * 365 + (self.month as u64) * 30 + (self.day as u64)
    }
}