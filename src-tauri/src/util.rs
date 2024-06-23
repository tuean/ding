use std::{fs };

use tauri::api::path::data_dir;

pub fn get_db_file_path() -> String {
    let mut data_path: std::path::PathBuf = data_dir().unwrap();
    data_path.push("ding");
    let _ = fs::create_dir(&data_path);
    data_path.push("clipboard.db");
    let path = data_path.as_path().display().to_string();
    path
}

pub fn get_log_file_path() -> String {
    let mut data_path: std::path::PathBuf = data_dir().unwrap();
    data_path.push("ding");
    data_path.push("logs");
    let _ =  fs::create_dir(&data_path);
    data_path.push("log.{}.log");
    data_path.display().to_string()
}