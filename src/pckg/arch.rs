use crate::installer::Installer;
use log::error;
use std::process::Command;

pub struct Arch;

impl Installer for Arch {
    fn all(&self, root: &str) -> Vec<String> {
        vec![
            format!("{} pacman -Syu python-pip wine-staging winetricks lutris steam gamemode giflib lib32-giflib libpng lib32-libpng libldap lib32-libldap gnutls lib32-gnutls mpg123 lib32-mpg123 openal lib32-openal v4l-utils lib32-v4l-utils libpulse lib32-libpulse libgpg-error lib32-libgpg-error alsa-plugins lib32-alsa-plugins alsa-lib lib32-alsa-lib libjpeg-turbo lib32-libjpeg-turbo sqlite lib32-sqlite libxcomposite lib32-libxcomposite libxinerama lib32-libgcrypt libgcrypt lib32-libxinerama ncurses lib32-ncurses opencl-icd-loader lib32-opencl-icd-loader libxslt lib32-libxslt libva lib32-libva gtk3 lib32-gtk3 gst-plugins-base-libs lib32-gst-plugins-base-libs vulkan-icd-loader lib32-vulkan-icd-loader -y --needed --noconfirm", root),
        ]
    }

    fn gaming(&self, root: &str) -> Vec<String> {
        vec![
            format!("{} pacman -Syu python-pip wine-staging winetricks steam gamemode giflib lib32-giflib libpng lib32-libpng libldap lib32-libldap gnutls lib32-gnutls mpg123 lib32-mpg123 openal lib32-openal v4l-utils lib32-v4l-utils libpulse lib32-libpulse libgpg-error lib32-libgpg-error alsa-plugins lib32-alsa-plugins alsa-lib lib32-alsa-lib libjpeg-turbo lib32-libjpeg-turbo sqlite lib32-sqlite libxcomposite lib32-libxcomposite libxinerama lib32-libgcrypt libgcrypt lib32-libxinerama ncurses lib32-ncurses opencl-icd-loader lib32-opencl-icd-loader libxslt lib32-libxslt libva lib32-libva gtk3 lib32-gtk3 gst-plugins-base-libs lib32-gst-plugins-base-libs vulkan-icd-loader lib32-vulkan-icd-loader -y --needed --noconfirm", root),
        ]
    }

    fn lutris(&self, root: &str) -> Vec<String> {
        vec![format!("{} pacman -Syu lutris", root)]
    }

    fn heroic_launcher(&self, _: &str) -> Vec<String> {
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

    fn overlay(&self, _: &str) -> Vec<String> {
        let aur = get_aur_package_manager();

        if aur.is_empty() {
            error!("No AUR package manager found for this Arch distro");
            vec![]
        } else {
            vec![format!("{} goverlay-bin -y --needed --noconfirm", aur)]
        }
    }

    fn replay_sorcery(&self, _: &str) -> Vec<String> {
        let aur = get_aur_package_manager();

        if aur.is_empty() {
            error!("No AUR package manager found for this Arch distro");
            vec![]
        } else {
            vec![format!("{} replay-sorcery -y --needed --noconfirm", aur)]
        }
    }

    fn mini_galaxy(&self, _: &str) -> Vec<String> {
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
        .unwrap_or(&"")
        .to_string()
}
