use crate::log;
use home::home_dir;
use nix::unistd::geteuid;
use std::fs;
use std::path::Path;

pub struct Steam {
    pub _path: String,
    pub _proton_path: String,
    pub _proton_version: Vec<String>,
}

impl Steam {
    pub fn new() -> Result<Self, &'static str> {
        if geteuid().is_root() {
            Err("root privileged detected")
        } else {
            let steam_path = Steam::fpath();
            let statue = match steam_path {
                Ok(st_path) => {
                    let proton_path = Steam::ppath(&st_path);
                    Ok(Self {
                        _path: st_path.clone(),
                        _proton_path: proton_path.clone(),
                        _proton_version: Steam::all_proton_version(&proton_path)
                            .unwrap_or_default(),
                    })
                }
                Err(err) => Err(err),
            };

            statue
        }
    }

    // find steam path
    fn fpath() -> Result<String, &'static str> {
        let home_dir = home_dir().unwrap_or_default().display().to_string();
        let mut steam_path = home_dir.clone();
        steam_path.push_str("/.steam/");

        if !Path::new(&steam_path).exists() {
            steam_path = home_dir;
            // TODO: find the path for flatpak steam
            steam_path.push_str("/flatpak");

            if !Path::new(&steam_path).exists() {
                return Err("Can't find any Steam directory");
            }
        }

        Ok(steam_path)
    }

    // Parse steam apth to get proton path
    fn ppath(steam_path: &String) -> String {
        let mut proton_path = steam_path.clone();
        proton_path.push_str("root/compatibilitytools.d/");
        if !Path::new(&proton_path).exists() {
            match fs::create_dir_all(&proton_path).is_ok() {
                true => log::success(&format!(
                    "compatibilitytools.d directory is create at {}",
                    steam_path
                )),
                false => {
                    log::error(&format!(
                        "Can't create compatibilitytools.d directory on this directory {}",
                        steam_path
                    ));
                    log::error("Try to open Steam for create directory");
                }
            }
        }

        return proton_path.to_string();
    }

    fn all_proton_version(proton_path: &String) -> std::io::Result<Vec<String>> {
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
        self._proton_version.contains(version)
    }
}
