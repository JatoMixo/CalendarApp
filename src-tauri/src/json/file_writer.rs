use std::{fs::File, io::Write};
use serde_json::Value;
use crate::error::FileError;
use crate::json::json_formatter::json_to_string;

pub fn write_to_file(file_path: &str, content: String) -> Result<(), FileError> {

    if !std::path::Path::new(file_path).exists() {
        match File::create(file_path) {
            Ok(_new_file) => {},
            Err(_error) => return Err(FileError::CreateError),
        }
    }

    let mut file = File::open(file_path).unwrap();
    match file.write_all(content.as_bytes()) {
        Ok(_value) => Ok(()),
        Err(_error) => Err(FileError::WriteError),
    }
}

pub fn write_json_to_file(file_path: &str, json_content: Value) -> Result<(), FileError> {

    let json_string = json_to_string(json_content);

    write_to_file(file_path, json_string)
}