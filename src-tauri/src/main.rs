// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod json;

use json::json_formatter::{json_from_string, json_to_string};
use json::{file_reader, file_writer, error};

mod user_folder;
use user_folder::{get_user_folder, CACHE_NAME};

mod cache;
use cache::read_cache;

use serde_json::json;

fn main() {
  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
