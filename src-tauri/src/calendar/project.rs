use serde::{Serialize, Deserialize};
use crate::calendar::date::Date;

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    name: String,
    color: String,
    description: String,
    start_date: Date,
    final_date: Date,
}