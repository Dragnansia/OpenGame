use crate::{
    downloader,
    error::{dir, unv},
    steam::Steam,
    timer,
};
use log::{debug, info, warn};
use serde_json::Value;
use std::{
    fs::{self, File},
    path::Path,
};
use tar::Archive;

const GITHUB_API: &str = "https://api.github.com/repos/GloriousEggroll/proton-ge-custom/releases";

pub fn remove_cache() -> Result<(), dir::Error> {
    let po = crate::dir::format_tmp_dir("proton", false)?;
    let path = Path::new(&po);

    if path.exists() {
        fs::remove_dir_all(path)?;
        fs::create_dir_all(&path)?;
        info!("Cache folder for ProtonGE is removed");
    }

    Ok(())
}

pub async fn install_version(version_name: &str, steam: &Steam) -> Result<(), unv::Error> {
    let releases = downloader::get(GITHUB_API).await?;
    let arr = releases.as_array().ok_or("No releases array")?;

    for r in arr {
        let tag_name = r["tag_name"].as_str().ok_or("No tag_name")?;
        if tag_name.starts_with(version_name)
            && !steam.is_installed(&format!("Proton-{}", tag_name))
        {
            let timer = timer::current_time();

            let assets = r["assets"]
                .as_array()
                .ok_or("Can't convert asset to array")?;

            if assets.is_empty() {
                warn!("{} don't have any assets to download", tag_name);
                break;
            }

            download_and_install_proton(assets, steam).await?;

            info!(
                "{} installation done ({} secs)",
                tag_name,
                timer::get_duration(&timer)
            );

            return Ok(());
        }
    }

    Err("No version found with this name".into())
}

pub fn install_archive_version(path: &str, steam: &Steam) -> Result<(), unv::Error> {
    let tar_gz = File::create(path)?;
    let mut archive = Archive::new(tar_gz);

    debug!("Extract {}", &path);
    let timer = timer::current_time();
    archive.unpack(&steam.proton_path).unwrap();

    info!(
        "{} unzip done ({} sec(s))",
        path,
        timer::get_duration(&timer)
    );

    Ok(())
}

async fn download_and_install_proton(assets: &Vec<Value>, steam: &Steam) -> Result<(), unv::Error> {
    for asset in assets {
        let name = asset["name"].as_str().ok_or("No name on asset")?;
        if name.ends_with(".tar.gz") {
            let path = crate::dir::format_tmp_dir("proton", true)?;
            let final_path = format!("{}{}", path, name);

            let url = asset["browser_download_url"]
                .as_str()
                .ok_or("No browser_download_url")?;

            let timer = timer::current_time();

            downloader::download_file(url, &final_path).await?;

            info!(
                "{} is download ({} sec(s))",
                name,
                timer::get_duration(&timer)
            );

            install_archive_version(&final_path, steam)?;
            break;
        }
    }

    Ok(())
}

pub async fn update_protonge(steam: &Steam) -> Option<()> {
    let url = format!("{}{}", GITHUB_API, "?per_page=1");
    if let Ok(res) = downloader::get(&url).await {
        let last_release = &res.as_array()?[0];

        let name_release = last_release["tag_name"].as_str()?;
        if steam.is_installed(&format!("Proton-{}", name_release)) {
            warn!("The latest ProtonGE version is already installed")
        } else {
            let assets = last_release["assets"].as_array()?;
            download_and_install_proton(assets, steam).await.ok()?;
            info!("Installation of {} is finished", name_release);
        }
    }

    Some(())
}

pub fn remove_version(version_name: &str, steam: &Steam) -> Result<(), dir::Error> {
    let folder_name = format!("Proton-{}", version_name);
    if steam.is_installed(&folder_name) {
        fs::remove_dir_all(&format!("{}{}", steam.proton_path, &folder_name))?;
        info!("{} is removed", version_name);
    } else {
        warn!("{} is not installed", version_name);
    }

    Ok(())
}

pub fn list_version(steam: &Steam) {
    let proton_version = &steam.proton_version;

    if proton_version.is_empty() {
        warn!("No Proton version installed");
    } else {
        debug!("Proton version installed:");
        for pe in proton_version {
            debug!("- {}", pe);
        }
    }
}
