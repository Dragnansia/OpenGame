use crate::log;
use home::home_dir;
use std::{fs, path::Path, process::exit};

const TMP_DIR: &str = "/.cache/opengame/";

pub fn format_tmp_dir(folder: &str, create_if_not_exist: bool) -> String {
    let dir = format!("{}{}/", temp_dir(), folder);
    if create_if_not_exist && !Path::new(&dir).exists() {
        let _ = fs::create_dir_all(&dir);
    }

    dir
}

pub fn temp_dir() -> String {
    let hd = user_dir();
    let temp_dir = format!("{}{}", hd, TMP_DIR);

    if !Path::new(&temp_dir).exists() {
        let _ = fs::create_dir_all(&temp_dir);
        log::success("Create temp folder");
    }

    temp_dir
}

pub fn user_dir() -> String {
    let hd = home_dir();
    if hd.is_none() {
        log::error("No found home directory");
        exit(-1);
    }

    hd.unwrap_or_default()
        .as_path()
        .to_str()
        .unwrap_or_default()
        .to_string()
}
