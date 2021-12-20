use crate::{dir, downloader::get, installer::Installer, log::*};
use std::process::Command;

pub struct Ubuntu;

#[async_trait::async_trait]
impl Installer for Ubuntu {
    async fn all(&self, root: &String) -> Vec<String> {
        vec![
            format!("{} apt install -y software-properties-common", root),
            format!("{} dpkg --add-architecture i386", root),
            "wget -nc https://dl.winehq.org/wine-builds/winehq.key".to_string(),
            format!("{} apt-key add winehq.key", root),
            format!(
                "{} add-apt-repository 'deb https://dl.winehq.org/wine-builds/ubuntu/ {} main' -y",
                root,
                release_code_name()
            ),
            format!("{} add-apt-repository -y ppa:lutris-team/lutris", root),
            format!("{} add-apt-repository -y multiverse", root),
            format!("{} add-apt-repository -y ppa:flexiondotorg/mangohud", root),
            format!("{} apt update", root),
            format!(
                "{} apt install --install-recommends winehq-staging -y",
                root
            ),
            format!("{} apt install -y steam goverlay winetricks lutris python3-pip gawk curl meson libsystemd-dev pkg-config ninja-build git libdbus-1-dev libinih-dev dbus-user-session libgnutls30:i386 libldap-2.4-2:i386 libgpg-error0:i386 libxml2:i386 libasound2-plugins:i386 libsdl2-2.0-0:i386 libfreetype6:i386 libdbus-1-3:i386 libsqlite3-0:i386", root),
        ]
    }

    async fn gaming(&self, root: &String) -> Vec<String> {
        vec![
            format!("{} apt install -y software-properties-common", root),
            format!("{} dpkg --add-architecture i386", root),
            "wget -nc https://dl.winehq.org/wine-builds/winehq.key".to_string(),
            format!("{} apt-key add winehq.key", root),
            format!(
                "{} add-apt-repository -y 'deb https://dl.winehq.org/wine-builds/ubuntu/ {} main'",
                root,
                release_code_name()
            ),
            format!("{} add-apt-repository -y multiverse", root),
            format!("{} apt update", root),
            format!(
                "{} apt install -y --install-recommends winehq-staging",
                root
            ),
            format!("{} apt install -y steam winetricks python3-pip gawk curl meson libsystemd-dev pkg-config ninja-build git libdbus-1-dev libinih-dev dbus-user-session libgnutls30:i386 libldap-2.4-2:i386 libgpg-error0:i386 libxml2:i386 libasound2-plugins:i386 libsdl2-2.0-0:i386 libfreetype6:i386 libdbus-1-3:i386 libsqlite3-0:i386", root),
        ]
    }

    async fn lutris(&self, root: &String) -> Vec<String> {
        vec![
            format!("{} apt install -y software-properties-common", root),
            format!("{} dpkg --add-architecture i386", root),
            format!("{} add-apt-repository -y ppa:lutris-team/lutris", root),
            format!("{} apt update", root),
            format!("{} apt install -y lutris", root),
        ]
    }

    async fn heroic_launcher(&self, _root: &String) -> Vec<String> {
        match dir::user_dir() {
            Ok(ud) => vec![
                "curl -o heroic.AppImage -LJO $(curl -s https://api.github.com/repos/Heroic-Games-Launcher/HeroicGamesLauncher/releases | grep browser_download_url | grep '[.]AppImage' | head -n 1 | cut -d '\"' -f 4)".to_string(),
                "chmod +x heroic.AppImage".to_string(),
                format!("mkdir -p {}/.applications", ud),
                format!("mv heroic.AppImage {}/.applications", ud),
                format!("{}/.applications/heroic.AppImage", ud)
            ],
            Err(err) => {
                error!("{}", err);
                vec![]
            }
        }
    }

    async fn overlay(&self, root: &String) -> Vec<String> {
        vec![
            format!("{} dpkg --add-architecture i386", root),
            format!("{} add-apt-repository -y ppa:flexiondotorg/mangohud", root),
            format!("{} apt update", root),
            format!("{} apt install -y goverlay", root),
        ]
    }

    async fn replay_sorcery(&self, root: &String) -> Vec<String> {
        let destination = format!(
            "{}ReplaySorcery",
            dir::format_tmp_dir("gaming", true).unwrap_or_default()
        );

        vec![
            format!(
                "{} apt install git cmake ffmpeg ffmpeg-libs ffmpeg libdrm libX11 libX11-xcb libX11",
                root
            ),
            format!(
                "git clone --recursive -j4 https://github.com/matanui159/ReplaySorcery.git {}",
                destination
            ),
            format!(
                "cmake -B {dest}/bin -S {dest} -DCMAKE_BUILD_TYPE=Release",
                dest=destination
            ),
            format!("make -C {}/bin", destination),
            format!("{} make -C {}/bin install", root, destination),
            "systemctl --user enable --now replay-sorcery".to_string(),
            format!("{} systemctl enable --now replay-sorcery-kms", root),
        ]
    }

    async fn mini_galaxy(&self, root: &String) -> Vec<String> {
        match dir::user_dir() {
            Ok(dir) => vec![
                format!(
                    "curl -o {}/minigalaxy.deb -LJO {}",
                    dir,
                    find_mini_galaxy_last_release().await
                ),
                format!("{} apt install {}/minigalaxy.deb", root, dir),
                format!("{} rm -f {}/minigalaxy.deb", root, dir),
            ],
            Err(err) => {
                error!("{}", err);
                vec![]
            }
        }
    }
}

// Todo: find a better way to return the result
async fn find_mini_galaxy_last_release() -> String {
    let res = get("https://api.github.com/repos/sharkwouter/minigalaxy/releases").await;
    let arr = res.as_array().unwrap();
    let assets = &arr[0]["assets"].as_array().unwrap()[0];
    let url = assets["browser_download_url"].as_str().unwrap_or_default();
    String::from(url)
}

fn release_code_name() -> String {
    match Command::new("lsb_release").arg("-cs").output() {
        Ok(res) => String::from_utf8(res.stdout).unwrap_or_default(),
        Err(err) => {
            error!("{:?}", err);
            String::new()
        }
    }
}
