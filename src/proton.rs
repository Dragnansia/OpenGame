use crate::{net, steam::Steam};

const GITHUB_API: &str = "https://api.github.com/repos/GloriousEggroll/proton-ge-custom/releases";

pub fn install_version(_version_name: &str, _steam: &Steam) {
    net::download_file(GITHUB_API);
}

pub fn remove_version(_version_name: &str, _steam: &Steam) {}

pub fn list_version(_steam: &Steam) {}
