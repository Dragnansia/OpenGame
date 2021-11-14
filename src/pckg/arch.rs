use super::installer::Installer;
use crate::dir;
use crate::log;
use std::process::{exit, Command};

pub struct Arch {}

// TODO: Verif if this arch gestion work properly
impl Installer for Arch {
    fn all(&self, root: &String) -> Vec<String> {
        [
            format!("{} pacman -Syu python-pip wine-staging winetricks lutris steam gamemode giflib lib32-giflib libpng lib32-libpng libldap lib32-libldap gnutls lib32-gnutls mpg123 lib32-mpg123 openal lib32-openal v4l-utils lib32-v4l-utils libpulse lib32-libpulse libgpg-error lib32-libgpg-error alsa-plugins lib32-alsa-plugins alsa-lib lib32-alsa-lib libjpeg-turbo lib32-libjpeg-turbo sqlite lib32-sqlite libxcomposite lib32-libxcomposite libxinerama lib32-libgcrypt libgcrypt lib32-libxinerama ncurses lib32-ncurses opencl-icd-loader lib32-opencl-icd-loader libxslt lib32-libxslt libva lib32-libva gtk3 lib32-gtk3 gst-plugins-base-libs lib32-gst-plugins-base-libs vulkan-icd-loader lib32-vulkan-icd-loader -y --needed --noconfirm", root),
        ].to_vec()
    }

    fn gaming(&self, root: &String) -> Vec<String> {
        [
            format!("{} pacman -Syu python-pip wine-staging winetricks steam gamemode giflib lib32-giflib libpng lib32-libpng libldap lib32-libldap gnutls lib32-gnutls mpg123 lib32-mpg123 openal lib32-openal v4l-utils lib32-v4l-utils libpulse lib32-libpulse libgpg-error lib32-libgpg-error alsa-plugins lib32-alsa-plugins alsa-lib lib32-alsa-lib libjpeg-turbo lib32-libjpeg-turbo sqlite lib32-sqlite libxcomposite lib32-libxcomposite libxinerama lib32-libgcrypt libgcrypt lib32-libxinerama ncurses lib32-ncurses opencl-icd-loader lib32-opencl-icd-loader libxslt lib32-libxslt libva lib32-libva gtk3 lib32-gtk3 gst-plugins-base-libs lib32-gst-plugins-base-libs vulkan-icd-loader lib32-vulkan-icd-loader -y --needed --noconfirm", root),
        ].to_vec()
    }

    fn lutris(&self, root: &String) -> Vec<String> {
        [format!("{} pacman -S lutris", root)].to_vec()
    }

    fn heroic_launcher(&self, _root: &String) -> Vec<String> {
        let aur = get_aur_package_manager();

        if aur.is_empty() {
            log::error("No Aur package manager found for this arch distro");
            exit(1);
        }

        [format!(
            "{} -S heroic-games-launcher-bin -y --needed --noconfirm",
            aur
        )]
        .to_vec()
    }

    fn overlay(&self, _root: &String) -> Vec<String> {
        let aur = get_aur_package_manager();

        if aur.is_empty() {
            log::error("No Aur package manager found for this arch distro");
            exit(1);
        }

        [format!("{} goverlay-bin -y --needed --noconfirm", aur)].to_vec()
    }

    fn replay_sorcery(&self, root: &String) -> Vec<String> {
        let destination = format!("{}ReplaySorcery", dir::format_tmp_dir("gaming", true));
        let build_dir = format!("{}/bin", destination);

        [
            format!("{} dnf install cmake ffmpeg-devel ffmpeg-libs ffmpeg libdrm libX11-devel libX11-xcb libX11", root),
            format!("git clone --recursive -j4 https://github.com/matanui159/ReplaySorcery.git {}", destination),
            format!("cmake -B {} -S {} -DCMAKE_BUILD_TYPE=Release", build_dir, destination),
            // Verif if this command is run correctly
            format!("make -C {}", build_dir),
            format!("{} make -C {} install", root, build_dir),
            "systemctl --user enable --now replay-sorcery".to_string(),
            format!("{} systemctl enable --now replay-sorcery-kms", root)
        ].to_vec()
    }
}

fn get_aur_package_manager() -> String {
    let aur_list = ["yay", "pamac", "paru"];
    let mut aur = String::new();

    for aurl in aur_list {
        let res = Command::new("command").arg("-v").arg(aurl).output();

        match res {
            Ok(_r) => {
                aur = aurl.to_string();
                log::success(&format!("Aur package Manager command is {}", aurl));
                break;
            }
            Err(_e) => {}
        }
    }

    aur
}
