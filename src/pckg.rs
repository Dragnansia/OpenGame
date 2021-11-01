mod fedora;
pub mod installer;

use crate::log;
use std::process::Command;

pub fn run_commands(cmds: &Vec<String>) {
    for command in cmds {
        let mut args: Vec<&str> = command.split(' ').collect();
        let cmd = args[0];
        args.remove(0);
        let res = Command::new(cmd).args(args).status();

        match res {
            Ok(_o) => log::success("No error on this command"),
            Err(_e) => log::error(&_e.to_string()),
        }
    }
}
