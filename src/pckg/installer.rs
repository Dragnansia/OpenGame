use crate::pckg::{arch::Arch, fedora::Fedora, ubuntu::Ubuntu};
use log::info;
use std::{
    io::{Error, ErrorKind},
    process::Command,
};

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
    let res = ["sudo", "doas", "su"]
        .iter()
        .find(|el| Command::new(el).output().is_ok())
        .unwrap_or(&"")
        .to_string();
    info!("Root command is {}", res);
    res
}

pub fn find_installer() -> Result<&'static dyn Installer, Error> {
    let output = Command::new("lsb_release").arg("-is").output()?;

    let distro_utf8 = String::from_utf8(output.stdout).unwrap_or_default();
    let distro_name = &distro_utf8[..distro_utf8.len() - 1];
    info!("Current distro is {}", distro_name);

    match distro_name {
        "Fedora" => Ok(&Fedora {}),
        "Arch" => Ok(&Arch {}),
        "Ubuntu" | "Elementary" => Ok(&Ubuntu {}),
        _ => Err(Error::new(ErrorKind::Other, "Can't find distro package")),
    }
}
