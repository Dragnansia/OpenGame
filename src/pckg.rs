mod arch;
mod fedora;
pub mod installer;
mod ubuntu;

use crate::log::*;
use std::process::Command;

pub fn run_commands(cmds: &Vec<String>) {
    for command in cmds {
        let mut args: Vec<&str> = command.split(' ').collect();
        let cmd = args[0];
        args.remove(0);
        let res = Command::new(cmd).args(&args).status();

        match res {
            Ok(s) => {
                if s.success() {
                    success!("No error on last command");
                } else {
                    error!("Command error code: {}", s.code().unwrap());
                }
            }
            Err(_e) => error!("{}", &_e.to_string()),
        }
    }
}
