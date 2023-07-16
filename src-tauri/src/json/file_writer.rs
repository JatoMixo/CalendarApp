use std::{fs::File, fs::OpenOptions, io::Write};
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

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)
        .unwrap();

    println!("Writing content to file: {}", content);
    match file.write_all(content.as_bytes()) {
        Ok(_value) => Ok(()),
        Err(_error) => {println!("{}", _error); Err(FileError::WriteError)},
    }
}

pub fn write_json_to_file(file_path: &str, json_content: Value) -> Result<(), FileError> {

    let json_string = json_to_string(json_content);
    println!("Created JSON String: {}", json_string);

    write_to_file(file_path, json_string)
}