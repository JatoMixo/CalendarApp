use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Date {
    day: u8,
    month: u8,
    year: u16,
}

impl Date {
    fn month_to_string(&self) -> Option<String> {
        const MONTHS: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

        if self.month as usize >= MONTHS.len() {
            return None;
        }

        Some(MONTHS[self.month as usize].to_string())
    }
}