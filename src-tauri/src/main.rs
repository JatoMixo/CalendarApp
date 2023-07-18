// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod json;

use json::json_formatter::{json_from_string, json_to_string};
use json::{file_reader, file_writer, error};

mod user_folder;
use user_folder::{get_user_folder, CACHE_NAME};

mod cache;
use cache::{read_json_cache, get_projects_from_cache, create_cache, push_project_to_cache};

mod calendar;
use calendar::project::Project;
use calendar::date::Date;

use serde_json::json;

fn main() {
  let project = Project {
    name: "Potato".to_string(),
    description: "ASDASD".to_string(),
    color: "#6f6f6f".to_string(),
    start_date: Date {
      day: 24,
      month: 6,
      year: 2023,
    },
    final_date: Date {
      day: 30,
      month: 7,
      year: 2077,
    },
  };
  println!("Created the cache: {:?}", create_cache());
  println!("Pushed the new project: {:?}", push_project_to_cache(project));
  println!("Projects from cache: {:?}", get_projects_from_cache());
  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
