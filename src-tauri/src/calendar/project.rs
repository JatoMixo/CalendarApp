use serde::{Serialize, Deserialize};

#[Derive(Debug, Serialize, Deserialize)]
pub struct Project {
    name: String,
    color: String,
    description: String,
    start_date: Date,
    final_date: Date,
}