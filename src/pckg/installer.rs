use crate::{
    error::unv::Error,
    pckg::{arch::Arch, fedora::Fedora, ubuntu::Ubuntu},
    utils::os_release_data,
};
use std::process::Command;

/// Function to install group dependencies
pub trait Installer {
    fn all(&self, root: &str) -> Vec<String>;
    fn gaming(&self, root: &str) -> Vec<String>;
    fn lutris(&self, root: &str) -> Vec<String>;
    fn heroic_launcher(&self, root: &str) -> Vec<String>;
    fn overlay(&self, root: &str) -> Vec<String>;
    fn replay_sorcery(&self, root: &str) -> Vec<String>;
    fn mini_galaxy(&self, root: &str) -> Vec<String>;
}

pub fn root_command() -> String {
    ["sudo", "doas", "su"]
        .iter()
        .find(|el| Command::new(el).output().is_ok())
        .unwrap_or(&"")
        .to_string()
}

pub fn find_installer() -> Result<(String, &'static dyn Installer), Error> {
    let distro_name = distro_name()?;
    let installer: &dyn Installer = match distro_name.as_str() {
        "Fedora" => &Fedora,
        "Arch" => &Arch,
        "Ubuntu" | "Elementary" => &Ubuntu,
        _ => return Err("Can't find distro package".into()),
    };

    Ok((distro_name, installer))
}

pub fn distro_name() -> Result<String, Error> {
    let (_, value) = os_release_data("NAME")?;
    let value: Vec<&str> = value.split(' ').collect();
    Ok(value[0].into())
}
