use std::fs::File;

use serde_json::{Value, json};
use crate::file_reader::json_from_file;
use crate::user_folder::{get_user_folder, CACHE_NAME};
use crate::file_writer::write_json_to_file;
use crate::calendar::project::Project;
use crate::json::error::FileError;

pub fn read_json_cache() -> Value {

    let cache_path = &(get_user_folder().unwrap() + CACHE_NAME);

    match json_from_file(&cache_path) {
        Ok(value) => value,
        Err(_) => {
            create_cache().unwrap();
            read_json_cache()
        }
    }
}

pub fn get_projects_from_cache() -> Vec<Project> {
    let projects_as_values: Vec<Value> = read_json_cache()["projects"].as_array().unwrap().to_vec();
    let mut projects: Vec<Project> = Vec::new();

    for project_value in projects_as_values {
        let project: Project = Project::from_json(project_value).unwrap();

        projects.push(project);
    }

    projects
}

pub fn push_project_to_cache(project: Project) -> Result<(), FileError> {

    let mut projects: Vec<Project> = get_projects_from_cache();

    for project_i in projects.iter() {
        if project_i.name == project.name {
            return Err(FileError::ProjectAlreadyExists);
        }
    }

    projects.push(project);

    let json_cache = json!({
        "projects": projects,
    });

    let mut cache_path: String = String::new();

    match get_user_folder() {
        Ok(user_folder) => cache_path = user_folder + CACHE_NAME,
        Err(_) => return Err(FileError::CreateError),
    }

    write_json_to_file(&cache_path, json_cache)
}

pub fn remove_project_from_cache(project_name: String) -> Result<(), FileError> {

    let actual_projects = get_projects_from_cache();
    create_cache().unwrap();

    for project in actual_projects {
        if project.name == project_name {
            continue;
        }

        push_project_to_cache(project).unwrap();
    }

    Ok(())
}

pub fn create_cache() -> Result<(), FileError> {

    let cache_structure = json!({
        "projects": [

        ]
    });

    let user_folder: String;

    match get_user_folder() {
        Ok(new_user_folder) => user_folder = new_user_folder,
        Err(_) => return Err(FileError::CreateError),
    }

    let cache_path = user_folder + CACHE_NAME;

    write_json_to_file(&cache_path, cache_structure)
}