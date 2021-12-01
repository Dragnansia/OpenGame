use crate::{
    log::*,
    pckg::{arch::Arch, fedora::Fedora},
};
use std::{
    io::{Error, ErrorKind},
    process::Command,
};

pub trait Installer {
    fn all(&self, root: &String) -> Vec<String>;
    fn gaming(&self, root: &String) -> Vec<String>;
    fn lutris(&self, root: &String) -> Vec<String>;
    fn heroic_launcher(&self, root: &String) -> Vec<String>;
    fn overlay(&self, root: &String) -> Vec<String>;
    fn replay_sorcery(&self, root: &String) -> Vec<String>;
    fn mini_galaxy(&self, root: &String) -> Vec<String>;
}

pub fn root_command() -> String {
    let res = ["sudo", "doas", "su"]
        .iter()
        .find(|el| Command::new("command").arg("-v").arg(el).output().is_ok())
        .unwrap_or_else(|| &"")
        .to_string();
    success!("Root command is {}", res);
    res
}

pub fn find_installer() -> Result<Box<dyn Installer>, Error> {
    match Command::new("lsb_release").arg("-is").output() {
        Ok(r) => {
            let distro_utf8 = String::from_utf8(r.stdout).unwrap_or_default();
            let distro_name = &distro_utf8[..distro_utf8.len() - 1];
            success!("Current distro is {}", distro_name);

            match distro_name {
                "Fedora" => Ok(Box::new(Fedora {})),
                "Arch" => Ok(Box::new(Arch {})),
                _ => Err(Error::new(ErrorKind::Other, "Can't find distro package")),
            }
        }
        Err(e) => Err(e),
    }
}
