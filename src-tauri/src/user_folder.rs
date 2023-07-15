use dirs::home_dir;

pub fn get_user_folder() -> Result<String, String> {
    match home_dir() {
        Some(home_directory) => Ok(home_directory.into_os_string().to_str().unwrap().to_string()),
        None => Err("Unable to get home directory".to_string()),
    }
}