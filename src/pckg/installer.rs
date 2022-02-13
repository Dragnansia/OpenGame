use crate::{
    error::unv::Error,
    pckg::{arch::Arch, fedora::Fedora, ubuntu::Ubuntu},
    utils::scan,
};
use log::info;
use std::{
    fs::File,
    io::{BufRead, BufReader},
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
    let distro_name = distro_name()?;
    info!("Current distro is {}", distro_name);

    match distro_name.as_str() {
        "Fedora" => Ok(&Fedora {}),
        "Arch" => Ok(&Arch {}),
        "Ubuntu" | "Elementary" => Ok(&Ubuntu {}),
        _ => Err("Can't find distro package".into()),
    }
}

fn distro_name() -> Result<String, Error> {
    let file = File::open("/etc/os-release")?;

    let reader = BufReader::new(file);
    for (_, line) in reader.lines().enumerate() {
        let line = line?;

        let (name, value) = scan!(&line, "=", String, String);

        let name = name.ok_or("No NAME value")?;
        if name != "NAME" {
            continue;
        }

        let value = value.ok_or("Value is Empty")?.replace("\"", "");
        let value: Vec<&str> = value.split(' ').collect();

        return Ok(value[0].into());
    }

    Err("No found NAME value".into())
}
