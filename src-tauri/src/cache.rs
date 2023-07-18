use std::fs::File;

use serde_json::{Value, json};
use crate::file_reader::json_from_file;
use crate::user_folder::{get_user_folder, CACHE_NAME};
use crate::file_writer::write_json_to_file;
use crate::calendar::project::Project;
use crate::json::error::FileError;

pub fn read_json_cache() -> Value {

    let cache_path = &(get_user_folder().unwrap() + CACHE_NAME);

    let mut json_data: Value = Value::Null;

    match json_from_file(&cache_path) {
        Ok(value) => {
            json_data = value;
        },
        Err(_error) => {
            create_cache().unwrap();
            return read_json_cache();
        },
    }

    json_data
}

pub fn get_projects_from_cache() -> Vec<Project> {
    let projects_as_values: Vec<Value> = read_json_cache()["projects"].as_array().unwrap().to_vec();
    let mut projects: Vec<Project> = Vec::new();

    for project_value in projects_as_values {
        let project: Project = serde_json::from_value(project_value).unwrap();

        projects.push(project);
    }

    projects
}

pub fn push_project_to_cache() -> Result<(), FileError> {
    Ok(())
}

pub fn remove_project_from_cache() -> Result<(), FileError> {
    Ok(())
}

pub fn create_cache() -> Result<(), FileError> {

    let cache_structure = json!({
        "projects": [

        ]
    });

    let cache_path = &(get_user_folder().unwrap() + CACHE_NAME);

    Ok(())
}