use crate::log::*;
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
        Err(_) => None,
    }
}

pub fn temp_dir() -> Result<String, &'static str> {
    match user_dir() {
        Ok(hd) => {
            let temp_dir = format!("{}{}", hd, TMP_DIR);

            if !Path::new(&temp_dir).exists() {
                let _ = fs::create_dir_all(&temp_dir);
                success!("Create temp directory");
            }

            Ok(temp_dir)
        }
        Err(err) => Err(err),
    }
}

pub fn user_dir() -> Result<String, &'static str> {
    match std::env::var("HOME") {
        Ok(hd) => Ok(hd),
        Err(_) => Err("Can't find home directory"),
    }
}
