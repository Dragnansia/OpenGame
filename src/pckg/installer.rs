use crate::{
    log,
    pckg::{arch::Arch, fedora::Fedora},
};
use std::process::{exit, Command};

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
                log::success(format!("root command is {}", root));
                break;
            }
            Err(_e) => {}
        }
    }

    rt
}

pub fn find_installer() -> Box<dyn Installer> {
    let res = Command::new("lsb_release").arg("-is").output();
    let mut installer: Option<Box<dyn Installer>> = None;

    match res {
        Ok(r) => {
            let distro_utf8 = String::from_utf8(r.stdout).unwrap_or_default();
            let distro_name = &distro_utf8[..distro_utf8.len() - 1];
            log::success(format!("Current distro is {}", distro_name));

            installer = match distro_name {
                "Fedora" => Some(Box::new(Fedora {})),
                "Arch" => Some(Box::new(Arch {})),
                _ => None,
            }
        }
        Err(e) => log::error(&e.to_string()),
    }

    if installer.is_none() {
        log::error("Can't find a gaming dependencies for this distro");
        exit(-1);
    }

    installer.unwrap()
}
