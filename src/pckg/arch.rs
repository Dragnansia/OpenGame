use crate::{installer::Installer, log::*};
use std::process::Command;

pub struct Arch;

#[async_trait::async_trait]
impl Installer for Arch {
    async fn all(&self, root: &String) -> Vec<String> {
        vec![
            format!("{} pacman -Syu python-pip wine-staging winetricks lutris steam gamemode giflib lib32-giflib libpng lib32-libpng libldap lib32-libldap gnutls lib32-gnutls mpg123 lib32-mpg123 openal lib32-openal v4l-utils lib32-v4l-utils libpulse lib32-libpulse libgpg-error lib32-libgpg-error alsa-plugins lib32-alsa-plugins alsa-lib lib32-alsa-lib libjpeg-turbo lib32-libjpeg-turbo sqlite lib32-sqlite libxcomposite lib32-libxcomposite libxinerama lib32-libgcrypt libgcrypt lib32-libxinerama ncurses lib32-ncurses opencl-icd-loader lib32-opencl-icd-loader libxslt lib32-libxslt libva lib32-libva gtk3 lib32-gtk3 gst-plugins-base-libs lib32-gst-plugins-base-libs vulkan-icd-loader lib32-vulkan-icd-loader -y --needed --noconfirm", root),
        ]
    }

    async fn gaming(&self, root: &String) -> Vec<String> {
        vec![
            format!("{} pacman -Syu python-pip wine-staging winetricks steam gamemode giflib lib32-giflib libpng lib32-libpng libldap lib32-libldap gnutls lib32-gnutls mpg123 lib32-mpg123 openal lib32-openal v4l-utils lib32-v4l-utils libpulse lib32-libpulse libgpg-error lib32-libgpg-error alsa-plugins lib32-alsa-plugins alsa-lib lib32-alsa-lib libjpeg-turbo lib32-libjpeg-turbo sqlite lib32-sqlite libxcomposite lib32-libxcomposite libxinerama lib32-libgcrypt libgcrypt lib32-libxinerama ncurses lib32-ncurses opencl-icd-loader lib32-opencl-icd-loader libxslt lib32-libxslt libva lib32-libva gtk3 lib32-gtk3 gst-plugins-base-libs lib32-gst-plugins-base-libs vulkan-icd-loader lib32-vulkan-icd-loader -y --needed --noconfirm", root),
        ]
    }

    async fn lutris(&self, root: &String) -> Vec<String> {
        vec![format!("{} pacman -Syu lutris", root)]
    }

    async fn heroic_launcher(&self, _root: &String) -> Vec<String> {
        let aur = get_aur_package_manager();

        if aur.is_empty() {
            error!("No AUR package manager found for this Arch distro");
            vec![]
        } else {
            vec![format!(
                "{} -S heroic-games-launcher-bin -y --needed --noconfirm",
                aur
            )]
        }
    }

    async fn overlay(&self, _root: &String) -> Vec<String> {
        let aur = get_aur_package_manager();

        if aur.is_empty() {
            error!("No AUR package manager found for this Arch distro");
            vec![]
        } else {
            vec![format!("{} goverlay-bin -y --needed --noconfirm", aur)]
        }
    }

    async fn replay_sorcery(&self, _root: &String) -> Vec<String> {
        let aur = get_aur_package_manager();

        if aur.is_empty() {
            error!("No AUR package manager found for this Arch distro");
            vec![]
        } else {
            vec![format!("{} replay-sorcery -y --needed --noconfirm", aur)]
        }
    }

    async fn mini_galaxy(&self, _root: &String) -> Vec<String> {
        let aur = get_aur_package_manager();

        if aur.is_empty() {
            error!("No AUR package manager found for this Arch distro");
            vec![]
        } else {
            vec![format!("{} minigalaxy -y --needed --noconfirm", aur)]
        }
    }
}

fn get_aur_package_manager() -> String {
    ["yay", "pamac", "paru"]
        .iter()
        .find(|el| Command::new("command").arg("-v").arg(el).output().is_ok())
        .unwrap_or_else(|| &"")
        .to_string()
}
