use serde_json::Value;

use super::error::FileError;

pub fn json_to_string(json_object: Value) -> String {
    json_object.to_string()
}

pub fn json_from_string(string: String) -> Result<Value, FileError> {
    // serde_json::from_str(&string).unwrap()

    match serde_json::from_str(&string) {
        Ok(x) => Ok(x),
        Err(x) => Err(FileError::ParseError),
    }
}