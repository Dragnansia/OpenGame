use crate::log;
use home::home_dir;
use std::{fs, path::Path};

const TMP_DIR: &str = "/.cache/opengame/";

pub fn format_tmp_dir(folder: &str, create_if_not_exist: bool) -> Option<String> {
    match temp_dir() {
        Ok(td) => {
            let dir = format!("{}{}/", &td, folder);
            if create_if_not_exist && !Path::new(&dir).exists() {
                let _ = fs::create_dir_all(&dir);
            }

            Some(dir)
        }
        Err(_err) => None,
    }
}

pub fn temp_dir() -> Result<String, &'static str> {
    match user_dir() {
        Ok(hd) => {
            let temp_dir = format!("{}{}", hd, TMP_DIR);

            if !Path::new(&temp_dir).exists() {
                let _ = fs::create_dir_all(&temp_dir);
                log::success("Create temp folder");
            }

            Ok(temp_dir)
        }
        Err(err) => Err(err),
    }
}

pub fn user_dir() -> Result<String, &'static str> {
    match home_dir() {
        Some(hd) => Ok(hd.as_path().to_str().unwrap_or_default().to_string()),
        None => Err("Can't find home directory"),
    }
}
