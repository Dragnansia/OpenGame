use crate::{net, steam::Steam};
use home::home_dir;
use std::fs;
use std::path::Path;

const GITHUB_API: &str = "https://api.github.com/repos/GloriousEggroll/proton-ge-custom/releases";
const TMP_DIR: &str = "/.local/share/og/tmp/";

pub fn install_version(_version_name: &str, _steam: &Steam) {
    let releases = net::get(GITHUB_API);
    let arr = releases.as_array().unwrap();

    for r in arr {
        if r["tag_name"].as_str().unwrap().starts_with(_version_name)
            && !_steam.is_installed(&_version_name.to_string())
        {
            let assets = r["assets"].as_array().unwrap();

            for a in assets {
                let name = a["name"].as_str().unwrap();
                if name.ends_with(".tar.gz") {
                    let mut path = String::from("");
                    match home_dir() {
                        Some(dir) => {
                            path = format!("{}{}", dir.to_str().unwrap().to_string(), TMP_DIR)
                        }
                        None => println!(""),
                    }

                    if !Path::new(&path).exists() {
                        let _ = fs::create_dir_all(TMP_DIR);
                    }

                    let url = a["browser_download_url"].as_str().unwrap().clone();
                    net::download_file(url, &format!("{}{}", path, name));

                    break;
                }
            }

            break;
        }
    }
}

pub fn remove_version(_version_name: &str, _steam: &Steam) {}

pub fn list_version(_steam: &Steam) {}
