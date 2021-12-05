use crate::{dir, log::*, net, steam::Steam};
use flate2::read::GzDecoder;
use serde_json::Value;
use std::{
    fs::{self, File},
    path::Path,
};
use tar::Archive;

const GITHUB_API: &str = "https://api.github.com/repos/GloriousEggroll/proton-ge-custom/releases";

pub fn remove_cache() -> Option<()> {
    let po = dir::format_tmp_dir("proton", false);
    let p = po?;
    let path = Path::new(&p);

    if path.exists() {
        let res = fs::remove_dir_all(path);

        match res {
            Ok(_r) => {
                let _ = fs::create_dir_all(&path);
                success!("Cache folder for ProtonGE is removed");
            }
            Err(err) => error!("Can't remove cache folder: {}", err),
        }
    }

    Some(())
}

pub fn install_version(_version_name: &str, _steam: &Steam) {
    let releases = net::get(GITHUB_API);
    let arr = releases.as_array().unwrap();

    for r in arr {
        let tag_name = r["tag_name"].as_str().unwrap();
        if tag_name.starts_with(_version_name)
            && !_steam.is_installed(&format!("Proton-{}", tag_name))
        {
            let assets = r["assets"].as_array().unwrap();
            download_and_install_proton(assets, _steam);

            success!("Installation of {} is finish", tag_name);
            break;
        }
    }
}

pub fn install_archive_version(path: &str, _steam: &Steam) {
    let tar_gz = File::open(&path).unwrap();
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    log!("Extract {}", &path);
    let _ = archive.unpack(&_steam.proton_path);
    success!("Installation of {} is finish", path);
}

fn download_and_install_proton(assets: &Vec<Value>, _steam: &Steam) -> Option<()> {
    for a in assets {
        let name = a["name"].as_str().unwrap();
        if name.ends_with(".tar.gz") {
            let path = dir::format_tmp_dir("proton", true);
            let final_path = format!("{}{}", path?, name);

            let url = a["browser_download_url"].as_str().unwrap();
            net::download_file(url, &final_path);
            install_archive_version(&final_path, _steam);

            break;
        }
    }

    Some(())
}

pub fn update_protonge(_steam: &Steam) {
    let res = net::get(&format!("{}{}", GITHUB_API, "?per_page=1"));
    let last_release = &res.as_array().unwrap()[0];

    let name_release = last_release["tag_name"].as_str().unwrap();

    match _steam.is_installed(&format!("Proton-{}", name_release)) {
        true => warning!("The latest ProtonGE version is already install"),
        false => {
            let assets = last_release["assets"].as_array().unwrap();
            download_and_install_proton(assets, _steam);
            success!("Installation of {} is finish", name_release);
        }
    }
}

pub fn remove_version(_version_name: &str, _steam: &Steam) {
    let folder_name = format!("Proton-{}", _version_name).to_string();
    if _steam.is_installed(&folder_name) {
        let res = fs::remove_dir_all(&format!("{}{}", _steam.proton_path, &folder_name));

        match res {
            Ok(()) => success!("{} is removed", _version_name),
            Err(err) => error!("{}", err.to_string()),
        }
    } else {
        warning!("{} is not install", _version_name);
    }
}

pub fn list_version(_steam: &Steam) {
    let proton_version = &_steam.proton_version;
    match proton_version.is_empty() {
        true => warning!("No Proton installed"),
        false => {
            log!("Proton version installed:");
            for pe in proton_version {
                log!("- {}", pe);
            }
        }
    }
}
