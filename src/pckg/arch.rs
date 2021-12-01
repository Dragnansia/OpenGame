use crate::{installer::Installer, log::*};
use std::process::Command;

pub struct Arch {}

// TODO: Need a verification on a arch distribution
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
        [format!("{} pacman -Syu lutris", root)].to_vec()
    }

    fn heroic_launcher(&self, _root: &String) -> Vec<String> {
        let aur = get_aur_package_manager();

        if aur.is_empty() {
            error!("No Aur package manager found for this arch distro");
            Vec::new()
        } else {
            [format!(
                "{} -S heroic-games-launcher-bin -y --needed --noconfirm",
                aur
            )]
            .to_vec()
        }
    }

    fn overlay(&self, _root: &String) -> Vec<String> {
        let aur = get_aur_package_manager();

        match aur.is_empty() {
            true => {
                error!("No Aur package manager found for this arch distro");
                Vec::new()
            }
            false => [format!("{} goverlay-bin -y --needed --noconfirm", aur)].to_vec(),
        }
    }

    fn replay_sorcery(&self, _root: &String) -> Vec<String> {
        let aur = get_aur_package_manager();

        match aur.is_empty() {
            true => {
                error!("No Aur package manager found for this arch distro");
                Vec::new()
            }
            false => [format!("{} replay-sorcery -y --needed --noconfirm", aur)].to_vec(),
        }
    }

    fn mini_galaxy(&self, _root: &String) -> Vec<String> {
        let aur = get_aur_package_manager();

        match aur.is_empty() {
            true => {
                error!("No Aur package manager found for this arch distro");
                Vec::new()
            }
            false => [format!("{} minigalaxy -y --needed --noconfirm", aur)].to_vec(),
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
