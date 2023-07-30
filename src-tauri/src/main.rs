// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod json;

use json::{file_reader, file_writer, error};

mod user_folder;

mod cache;
use cache::{get_projects_from_cache, get_project_from_date, add_project_to_cache_from_ui};

mod calendar;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_projects_from_cache, get_project_from_date, add_project_to_cache_from_ui])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
