use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Month {
    pub fn to_string(&self) -> String {
        match self {
            Month::January => "January",
            Month::February => "February",
            Month::March => "March",
            Month::April => "April",
            Month::May => "May",
            Month::June => "June",
            Month::July => "July",
            Month::August => "August",
            Month::September => "September",
            Month::October => "October",
            Month::November => "November",
            Month::December => "December",
        }.to_string()
    }

    pub fn from_string(string: String) -> Option<Month> {
        match string.as_str() {
            "January" => Some(Month::January),
            "February" => Some(Month::February),
            "March" => Some(Month::March),
            "April" => Some(Month::April),
            "May" => Some(Month::May),
            "June" => Some(Month::June),
            "July" => Some(Month::July),
            "August" => Some(Month::August),
            "September" => Some(Month::September),
            "October" => Some(Month::October),
            "November" => Some(Month::November),
            "December" => Some(Month::December),
            _ => None,
        }
    } 
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Date {
    day: u8,
    month: Month,
    year: u16,
}