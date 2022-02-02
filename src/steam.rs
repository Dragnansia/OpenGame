use crate::{error::dir, log::*};
use std::{fs, io, path::Path};

pub struct Steam {
    pub path: String,
    pub proton_path: String,
    pub proton_version: Vec<String>,
}

impl Steam {
    pub fn new() -> Result<Self, dir::Error> {
        let st_path = Steam::fpath()?;
        let proton_path = Steam::ppath(&st_path);
        Ok(Self {
            path: st_path,
            proton_path: proton_path.clone(),
            proton_version: Steam::all_proton_version(&proton_path).unwrap_or_default(),
        })
    }

    // find steam path
    fn fpath() -> Result<String, dir::Error> {
        let steam_path = format!("{}{}", crate::dir::user_dir()?, "/.steam/");

        match Path::new(&steam_path).exists() {
            true => Ok(steam_path),
            false => Err("Can't find any Steam directory".into()),
        }
    }

    // Parse steam path to get proton path
    fn ppath(steam_path: &String) -> String {
        let mut proton_path = steam_path.clone();
        proton_path.push_str("root/compatibilitytools.d/");
        if !Path::new(&proton_path).exists() {
            match fs::create_dir_all(&proton_path).is_ok() {
                true => success!("compatibilitytools.d directory is create at {}", steam_path),
                false => {
                    error!(
                        "Can't create compatibilitytools.d directory on this directory {}",
                        steam_path
                    );
                    error!("Try to open Steam for create directory");
                }
            }
        }

        proton_path.to_string()
    }

    fn all_proton_version(proton_path: &String) -> io::Result<Vec<String>> {
        let mut array: Vec<String> = Vec::new();
        for pe in fs::read_dir(proton_path)? {
            let pe = pe?;
            array.push(
                pe.path()
                    .file_name()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default()
                    .to_string(),
            );
        }

        array.sort();
        array.reverse();
        Ok(array)
    }

    pub fn is_installed(&self, version: &String) -> bool {
        self.proton_version.contains(version)
    }
}
