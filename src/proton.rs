use crate::{dir, downloader, log::*, steam::Steam};
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
            Ok(_) => {
                let _ = fs::create_dir_all(&path);
                success!("Cache folder for ProtonGE is removed");
            }
            Err(err) => error!("Can't remove cache folder: {}", err),
        }
    }

    Some(())
}

pub async fn install_version(version_name: &str, steam: &Steam) {
    let releases = downloader::get(GITHUB_API).await;
    let arr = releases.as_array().unwrap();

    for r in arr {
        let tag_name = r["tag_name"].as_str().unwrap();
        if tag_name.starts_with(version_name)
            && !steam.is_installed(&format!("Proton-{}", tag_name))
        {
            let assets = r["assets"].as_array().unwrap();
            download_and_install_proton(assets, steam).await.unwrap();

            success!("Installation of {} is finished", tag_name);
            break;
        }
    }
}

pub fn install_archive_version(path: &str, steam: &Steam) {
    let tar_gz = File::open(&path).unwrap();
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    log!("Extract {}", &path);
    archive.unpack(&steam.proton_path).unwrap();
    success!("Installation of {} is finished", path);
}

async fn download_and_install_proton(assets: &Vec<Value>, steam: &Steam) -> Option<()> {
    for a in assets {
        let name = a["name"].as_str().unwrap();
        if name.ends_with(".tar.gz") {
            let path = dir::format_tmp_dir("proton", true);
            let final_path = format!("{}{}", path?, name);

            let url = a["browser_download_url"].as_str().unwrap();
            downloader::download_file(url, &final_path).await;
            install_archive_version(&final_path, steam);

            break;
        }
    }

    Some(())
}

pub async fn update_protonge(steam: &Steam) {
    let res = downloader::get(&format!("{}{}", GITHUB_API, "?per_page=1")).await;
    let last_release = &res.as_array().unwrap()[0];

    let name_release = last_release["tag_name"].as_str().unwrap();

    match steam.is_installed(&format!("Proton-{}", name_release)) {
        true => warning!("The latest ProtonGE version is already installed"),
        false => {
            let assets = last_release["assets"].as_array().unwrap();
            download_and_install_proton(assets, steam).await;
            success!("Installation of {} is finished", name_release);
        }
    }
}

pub fn remove_version(version_name: &str, steam: &Steam) {
    let folder_name = format!("Proton-{}", version_name).to_string();
    if steam.is_installed(&folder_name) {
        let res = fs::remove_dir_all(&format!("{}{}", steam.proton_path, &folder_name));

        match res {
            Ok(()) => success!("{} is removed", version_name),
            Err(err) => error!("{}", err.to_string()),
        }
    } else {
        warning!("{} is not installed", version_name);
    }
}

pub fn list_version(steam: &Steam) {
    let proton_version = &steam.proton_version;
    match proton_version.is_empty() {
        true => warning!("No Proton version installed"),
        false => {
            log!("Proton version installed:");
            for pe in proton_version {
                log!("- {}", pe);
            }
        }
    }
}
