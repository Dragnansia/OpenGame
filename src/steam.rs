use crate::error::unv;
use log::{error, info};
use std::{fs, path::Path};

pub struct Steam {
    pub path: String,
    pub proton_path: String,
    pub proton_version: Vec<String>,
}

impl Steam {
    pub fn new() -> Result<Self, unv::Error> {
        let path = Steam::fpath()?;
        let proton_path = Steam::ppath(&path);
        Ok(Self {
            path,
            proton_version: Steam::all_proton_version(&proton_path)?,
            proton_path,
        })
    }

    // find steam path
    fn fpath() -> Result<String, unv::Error> {
        let steam_path = format!("{}{}", crate::dir::user_dir()?, "/.steam/");

        if Path::new(&steam_path).exists() {
            Ok(steam_path)
        } else {
            Err("Can't find any Steam directory".into())
        }
    }

    // Parse steam path to get proton path
    fn ppath(steam_path: &str) -> String {
        let proton_path = format!("{}root/compatibilitytools.d/", steam_path);

        if Path::new(&proton_path).exists() {
            return proton_path;
        }

        if fs::create_dir_all(&proton_path).is_ok() {
            info!("compatibilitytools.d directory is create at {}", steam_path);
        } else {
            error!(
                "Can't create compatibilitytools.d directory on this directory {}",
                steam_path
            );
            error!("Try to open Steam for create directory");
        }

        proton_path
    }

    fn all_proton_version(proton_path: &String) -> Result<Vec<String>, unv::Error> {
        let mut array: Vec<String> = Vec::new();
        for pe in fs::read_dir(proton_path)? {
            let pe = pe?;
            array.push(
                pe.path()
                    .file_name()
                    .ok_or("file name")?
                    .to_str()
                    .ok_or("to str")?
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
