use super::installer::Installer;
use crate::dir;
use crate::log;
use std::process::Command;

pub struct Fedora {}

impl Installer for Fedora {
    fn all(&self, root: &String) -> Vec<String> {
        let res = Command::new("lsb_release").arg("-rs").output();
        let mut fedora_version = String::new();

        match res {
            Ok(r) => {
                fedora_version = String::from_utf8(r.stdout).unwrap_or_default();
                log::success(&format!(
                    "Fedora version {}",
                    &fedora_version[..fedora_version.len() - 1]
                ));
            }
            Err(_e) => log::error("Can't get fedora version with lsb_release command"),
        }

        [
        format!("{} dnf install redhat-lsb-core -y", root),
        format!("{} dnf copr enable atim/heroic-games-launcher -y", root),
        format!("{} dnf update -y", root),
        format!("{} dnf config-manager --add-repo https://dl.winehq.org/wine-builds/fedora/{}/winehq.repo", root, fedora_version),
        format!("{} dnf install python3-pip wine-staging heroic-games-launcher-bin lutris gamemode goverlay steam alsa-plugins-pulseaudio.i686 glibc-devel.i686 glibc-devel libgcc.i686 libX11-devel.i686 freetype-devel.i686 libXcursor-devel.i686 libXi-devel.i686 libXext-devel.i686 libXxf86vm-devel.i686 libXrandr-devel.i686 libXinerama-devel.i686 mesa-libGLU-devel.i686 mesa-libOSMesa-devel.i686 libXrender-devel.i686 libpcap-devel.i686 ncurses-devel.i686 libzip-devel.i686 lcms2-devel.i686 zlib-devel.i686 libv4l-devel.i686 libgphoto2-devel.i686 cups-devel.i686 libxml2-devel.i686 openldap-devel.i686 libxslt-devel.i686 gnutls-devel.i686 libpng-devel.i686 flac-libs.i686 json-c.i686 libICE.i686 libSM.i686 libXtst.i686 libasyncns.i686 liberation-narrow-fonts.noarch libieee1284.i686 libogg.i686 libsndfile.i686 libuuid.i686 libva.i686 libvorbis.i686 libwayland-client.i686 libwayland-server.i686 llvm-libs.i686 mesa-dri-drivers.i686 mesa-filesystem.i686 mesa-libEGL.i686 mesa-libgbm.i686 nss-mdns.i686 ocl-icd.i686 pulseaudio-libs.i686 sane-backends-libs.i686 tcp_wrappers-libs.i686 unixODBC.i686 samba-common-tools.x86_64 samba-libs.x86_64 samba-winbind.x86_64 samba-winbind-clients.x86_64 samba-winbind-modules.x86_64 mesa-libGL-devel.i686 fontconfig-devel.i686 libXcomposite-devel.i686 libtiff-devel.i686 openal-soft-devel.i686 mesa-libOpenCL-devel.i686 opencl-utils-devel.i686 alsa-lib-devel.i686 gsm-devel.i686 libjpeg-turbo-devel.i686 pulseaudio-libs-devel.i686 pulseaudio-libs-devel gtk3-devel.i686 libattr-devel.i686 libva-devel.i686 libexif-devel.i686 libexif.i686 glib2-devel.i686 mpg123-devel.i686 mpg123-devel.x86_64 libcom_err-devel.i686 libcom_err-devel.x86_64 libFAudio-devel.i686 libFAudio-devel.x86_64 -y", root)
        ]
        .to_vec()
    }

    fn lutris(&self, root: &String) -> Vec<String> {
        [format!("{} dnf install lutris -y", root)].to_vec()
    }

    fn heroic_launcher(&self, root: &String) -> Vec<String> {
        [
            format!("{} dnf copr enable atim/heroic-games-launcher -y", root),
            format!("{} dnf update -y", root),
            format!("{} dnf install heroic-games-launcher-bin -y", root),
        ]
        .to_vec()
    }

    fn overlay(&self, root: &String) -> Vec<String> {
        [format!("{} dnf install goverlay -y", root)].to_vec()
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
