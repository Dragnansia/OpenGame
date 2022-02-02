use crate::{error::dir, log::*};
use std::{env::VarError, fs, path::Path};

const TMP_DIR: &str = "/.cache/opengame/";

pub fn format_tmp_dir(folder: &str, create_if_not_exist: bool) -> Result<String, dir::Error> {
    let temp_dir = temp_dir()?;
    let dir = format!("{}{}/", &temp_dir, folder);
    if create_if_not_exist && !Path::new(&dir).exists() {
        fs::create_dir_all(&dir)?;
    }

    Ok(dir)
}

pub fn temp_dir() -> Result<String, dir::Error> {
    let user_dir = user_dir()?;
    let temp_dir = format!("{}{}", user_dir, TMP_DIR);

    if !Path::new(&temp_dir).exists() {
        fs::create_dir_all(&temp_dir)?;
        success!("Create temp directory");
    }

    Ok(temp_dir)
}

pub fn user_dir() -> Result<String, VarError> {
    std::env::var("HOME")
}
