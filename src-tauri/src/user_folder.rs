use dirs::home_dir;

pub fn get_user_folder() -> Result<&str, &str> {
    match home_dir() {
        Some(home_directory) => Ok(home_directory.to_string()),
        None => Err("Unable to get home directory")
    }
}