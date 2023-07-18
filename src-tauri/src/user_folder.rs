use dirs::home_dir;

pub const CACHE_NAME: &str = "/projectcalendar-data.json";

pub fn get_user_folder() -> Result<String, String> {
    match home_dir() {
        Some(home_directory) => {
            let user_folder = home_directory.into_os_string().into_string().unwrap();

            let folder_remove_duplicates = user_folder.replace(r"\", r"/").to_string();

            return Ok(folder_remove_duplicates);
        },
        None => return Err("Unable to get home directory".to_string()),
    };
}