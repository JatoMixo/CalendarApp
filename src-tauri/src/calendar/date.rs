use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Date {
    pub day: u8,
    pub month: u8,
    pub year: u16,
}

impl Date {
    pub fn to_int(&self) -> u64 {
        (self.year as u64) * 365 + (self.month as u64) * 30 + (self.day as u64)
    }
}