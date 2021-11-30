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
}

pub fn root_command() -> String {
    let roots_list = ["sudo", "doas", "su"];
    let mut rt = String::new();

    for root in roots_list {
        let res = Command::new("command").arg("-v").arg(root).output();

        match res {
            Ok(_r) => {
                rt = root.to_string();
                success!("root command is {}", root);
                break;
            }
            Err(_e) => {}
        }
    }

    rt
}

pub fn find_installer() -> Result<Box<dyn Installer>, Error> {
    let res = Command::new("lsb_release").arg("-is").output();

    let installer: Result<Box<dyn Installer>, Error> = match res {
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
    };

    installer
}
