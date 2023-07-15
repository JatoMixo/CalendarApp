use std::fs::File;
use std::io::{copy, stdout};
use serde_json::Value;
use crate::json::json_formatter::json_from_string;
use crate::error::FileError;

pub fn read_file(file_path: &str) -> Result<String, FileError> {
    let mut file;

    match File::open(file_path) {
        Ok(x) => file = x,
        Err(_error) => return Err(FileError::ReadError),
    }

    match copy(&mut file, &mut stdout()) {
        Ok(x) => Ok(x.to_string()),
        Err(_error) => Err(FileError::ReadError),
    }
}

pub fn json_from_file(file_path: &str) -> Result<Value, FileError> {
    match read_file(file_path) {
        Ok(x) => match json_from_string(x) {
            Ok(final_json) => Ok(final_json),
            Err(_error) => Err(FileError::ParseError),
        },
        Err(_error) => return Err(FileError::ReadError),
    }
}