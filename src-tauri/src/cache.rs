use serde_json::{Value, json};
use crate::file_reader::json_from_file;
use crate::user_folder::{get_user_folder, CACHE_NAME};
use crate::file_writer::write_json_to_file;

pub fn read_cache() -> Value {

    let cache_structure: Value = json!({
        "projects": [

        ],
    });

    let cache_path = &(get_user_folder().unwrap() + CACHE_NAME);

    let mut json_data: Value = Value::Null;

    match json_from_file(&cache_path) {
        Ok(value) => {
            json_data = value;
        },
        Err(_error) => {
            match write_json_to_file(&cache_path, cache_structure) {
                Ok(_) => {},
                Err(_) => {},
            }
        },
    }

    json_data
}