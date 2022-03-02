use crate::{
    error::{dir, unv},
    timer,
};
use indicatif::{ProgressBar, ProgressStyle};
use lamodin::{archive, downloader::Download, launcher::steam::Steam, modifier::Modifier};
use log::{info, warn};
use std::{
    env::var,
    fs::{self},
    path::Path,
};

#[derive(Debug)]
pub struct ProtonDownload {
    total_bytes: u64,
    current_download: u64,
    pb: ProgressBar,
}

impl ProtonDownload {
    pub fn new(file_name: &str) -> Self {
        let pb = ProgressBar::new(1);
        pb.set_style(ProgressStyle::default_bar()
            .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
            .progress_chars("=- "));
        pb.set_message(format!("-> {}", file_name));

        Self {
            current_download: 0,
            total_bytes: 0,
            pb,
        }
    }
}

impl Download for ProtonDownload {
    fn init(&mut self, size: u64) {
        self.total_bytes = size;
        self.pb.set_length(size);
    }

    fn update(&mut self, chunk: &[u8]) {
        self.current_download = std::cmp::min(
            self.current_download + (chunk.len() as u64),
            self.total_bytes,
        );
        self.pb.set_position(self.current_download);
    }
}

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

pub fn install_archive_version(path: &str, steam: &Steam) -> Result<(), unv::Error> {
    let timer = timer::current_time();
    archive::install(path, &steam.modifier_path)?;

    info!(
        "{} unzip done ({} sec(s))",
        path,
        timer::get_duration(&timer)
    );

    Ok(())
}

pub async fn update_protonge(steam: &Steam) -> Result<(), unv::Error> {
    let versions = steam.versions().await?;
    let last_version = versions.first().ok_or("Version array is empty")?;

    if steam.is_installed(&format!("Proton-{}", last_version.tag_name)) {
        warn!("The latest ProtonGE version is already installed");
        return Ok(());
    }

    let timer = timer::current_time();

    let asset = last_version
        .assets
        .iter()
        .find(|asset| asset.name.ends_with("tar.gz"))
        .ok_or("Last ProtonGE version no found")?;

    let cache = lamodin::cache::path(&var("CARGO_PKG_NAME")?).ok_or("")?;
    let archive_path = format!("{}{}", cache, asset.name);

    lamodin::downloader::file(
        &asset.browser_download_url,
        &archive_path,
        &mut ProtonDownload::new(&asset.name),
    )
    .await?;

    install_archive_version(&archive_path, steam)?;
    info!("Installation of {} is finished", asset.name);

    info!(
        "{} installation done ({} secs)",
        last_version.tag_name,
        timer::get_duration(&timer)
    );

    Ok(())
}

pub fn remove_version(version_name: &str, steam: &Steam) -> Result<(), dir::Error> {
    let folder_name = format!("Proton-{}", version_name);
    if steam.is_installed(&folder_name) {
        fs::remove_dir_all(&format!("{}{}", steam.modifier_path, &folder_name))?;
        info!("{} is removed", version_name);
    } else {
        warn!("{} is not installed", version_name);
    }

    Ok(())
}

pub fn list_version(steam: &Steam) {
    let proton_version = &steam.modifiers;

    if proton_version.is_empty() {
        warn!("No Proton version installed");
    } else {
        info!("Proton version installed:");
        for pe in proton_version {
            info!("- {}", pe);
        }
    }
}
