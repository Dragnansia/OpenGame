mod arch;
mod fedora;
pub mod installer;

use crate::log;
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
                    log::success("No error on last command");
                } else {
                    log::error(&format!("Command error: {}", s.code().unwrap()));
                }
            }
            Err(_e) => log::error(&_e.to_string()),
        }
    }
}
