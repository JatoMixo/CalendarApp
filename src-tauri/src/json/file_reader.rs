use std::fs::File;
use std::io::Read;
use serde_json::Value;
use crate::json::json_formatter::json_from_string;
use crate::error::FileError;

pub fn read_file(file_path: &str) -> Result<String, FileError> {
    let mut file;

    match File::open(file_path) {
        Ok(x) => file = x,
        Err(_error) => return Err(FileError::ReadError),
    }

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_value) => Ok(content),
        Err(_error) => Err(FileError::ReadError),
    }
}

pub fn json_from_file(file_path: &str) -> Result<Value, FileError> {
    match read_file(file_path) {
        Ok(x) => json_from_string(x),
        Err(_error) => return Err(FileError::ReadError),
    }
}