use serde_json::{Value, json};
use crate::file_reader::json_from_file;
use crate::user_folder::{get_user_folder, CACHE_NAME};
use crate::file_writer::write_json_to_file;
use crate::calendar::project::Project;
use crate::calendar::date::Date;
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

#[tauri::command]
pub fn get_project_from_date(date: Date) -> Option<Project> {

    let projects = get_projects_from_cache();

    for project in projects {
        let is_date_in_project = date.to_int() >= project.start_date.to_int() && date.to_int() <= project.final_date.to_int();

        if is_date_in_project {
            return Some(project);
        }
    }

    None
}

#[tauri::command]
pub fn get_projects_from_cache() -> Vec<Project> {
    let projects_as_values: Vec<Value> = read_json_cache()["projects"].as_array().unwrap().to_vec();
    let mut projects: Vec<Project> = Vec::new();

    for project_value in projects_as_values {
        let project: Project = Project::from_json(project_value).unwrap();

        projects.push(project);
    }

    projects
}

#[tauri::command]
pub fn add_project_to_cache_from_ui(project: Project) {
    match push_project_to_cache(project) {
        Ok(()) => {},
        Err(_) => {},
    };
}

pub fn push_project_to_cache(project: Project) -> Result<(), FileError> {

    let mut projects: Vec<Project> = get_projects_from_cache();

    // Check if the project is valid
    for project_i in projects.iter() {

        // Check if the project already exists
        if project_i.name == project.name {
            return Err(FileError::ProjectAlreadyExists);
        }

        // Check if the dates are already in use
        let is_start_date_on_other_project: bool = project_i.start_date.to_int() <= project.start_date.to_int() && project_i.final_date.to_int() >= project.start_date.to_int();
        let is_final_date_on_other_project: bool = project_i.start_date.to_int() <= project.final_date.to_int() && project_i.final_date.to_int() >= project.final_date.to_int();
        let is_interfering_with_project: bool = project_i.start_date.to_int() < project.start_date.to_int() && project_i.final_date.to_int() > project.final_date.to_int();

        if is_start_date_on_other_project || is_final_date_on_other_project || is_interfering_with_project {
            return Err(FileError::InvalidDates);
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