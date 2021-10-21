use crate::{net, steam::Steam};
use flate2::read::GzDecoder;
use home::home_dir;
use std::fs::{self, File};
use std::path::Path;
use tar::Archive;

const GITHUB_API: &str = "https://api.github.com/repos/GloriousEggroll/proton-ge-custom/releases";
const TMP_DIR: &str = "/.local/share/og/tmp/";

pub fn install_version(_version_name: &str, _steam: &Steam) {
    let releases = net::get(GITHUB_API);
    let arr = releases.as_array().unwrap();

    for r in arr {
        let tag_name = r["tag_name"].as_str().unwrap();
        if tag_name.starts_with(_version_name)
            && !_steam.is_installed(format!("Proton-{}", tag_name).to_string())
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
                    let final_path = format!("{}{}", path, name);
                    net::download_file(url, &final_path);
                    install_archive_version(&final_path, _steam);

                    break;
                }
            }

            println!("-> Installation of {} is finish", tag_name);
            break;
        }
    }
}

pub fn install_archive_version(path: &str, _steam: &Steam) {
    let tar_gz = File::open(&path).unwrap();
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    println!("-> Extract {}", &path);
    let _ = archive.unpack(&_steam._proton_path);
    println!("-> Installation of {} is finish", path);
}

pub fn remove_version(_version_name: &str, _steam: &Steam) {}

pub fn list_version(_steam: &Steam) {
    println!("-> Proton version installed:");
    for pe in &_steam._proton_version {
        println!("{}", pe);
    }
}
