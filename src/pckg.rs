mod fedora;
pub mod installer;

use crate::log;
use std::process::Command;

pub fn run_commands(root: &str, cmds: &Vec<String>) {
    for command in cmds {
        let args: Vec<&str> = command.split(' ').collect();
        let res = Command::new(root).args(args).status();

        match res {
            Ok(_o) => log::success("No error on this command"),
            Err(_e) => log::error(&_e.to_string()),
        }
    }
}
