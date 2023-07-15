use serde_json::{Value};

pub fn json_to_string(json_object: Value) -> String {
    json_object.to_string()
}

pub fn json_from_string(string: String) -> Value {
    serde_json::from_str(&string).unwrap()
}