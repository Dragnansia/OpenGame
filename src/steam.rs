use std::path::Path;
use home::home_dir;
use std::fs;

pub struct Steam {
    _path: String,
    _proton_path: String,
    _proton_version: Vec<String>,
}

impl Steam {
    pub fn is_installed(&self, version: &String) -> bool {
        self._proton_version.contains(version)
    }
}

pub fn init_steam_data() -> Steam {
    let steam_path = find_steam_path();
    let proton_path = parse_proton_path(&steam_path);
    Steam {
        _path: steam_path.clone(),
        _proton_path: proton_path.clone(),
        _proton_version: get_all_proton_version_install(&proton_path).unwrap_or_default()
    }
}

fn find_steam_path() -> String {
    let home_dir = home_dir().unwrap_or_default().display().to_string();
    let mut steam_path = home_dir.clone();
    steam_path.push_str("/.steam/");

    //TODO: find the path for flatpak steam
    if !Path::new(&steam_path).exists() {
        steam_path = home_dir;
        steam_path.push_str("");
    }

    return steam_path;
}

fn parse_proton_path(_steam_path: &String) -> String {
    let mut proton_path = _steam_path.clone();
    proton_path.push_str("root/compatibilitytools.d/");
    if !Path::new(&proton_path).exists() {
        match fs::create_dir_all(proton_path.clone()).is_ok() {
            true => println!("-> compatibilitytools.d directory is create"),
            false => println!("-> Can't create compatibilitytools.d directory on steam folder")
        }
    }

    return proton_path.to_string();
}

fn get_all_proton_version_install(proton_path: &String) -> std::io::Result<Vec<String>> {
    let mut array: Vec<String> = Vec::new();
    for pe in fs::read_dir(proton_path)? {
        let pe = pe?;
        array.push(pe.path().file_name().unwrap_or_default().to_str().unwrap_or_default().to_string());
    }

    Ok(array)
}
