mod arch;
mod fedora;
pub mod installer;
mod ubuntu;

use crate::{log::*, timer};
use std::process::Command;

/// Run commands provided by a vector of string
///
/// # Examples
/// ```
/// let commands = [
///     "ls -al".to_string()
/// ].to_vec();
///
/// run_commands(&commands);
/// ```
pub fn run_commands(cmds: &Vec<String>) {
    for command in cmds {
        let timer = timer::current_time();

        let mut args: Vec<&str> = command.split(' ').collect();
        let cmd = args[0];
        args.remove(0);
        let res = Command::new(cmd).args(&args).status();

        match res {
            Ok(s) => {
                if s.success() {
                    success!(
                        "No error on last command ({} sec(s))",
                        timer::get_duration(&timer)
                    );
                } else {
                    error!("Command error code: {}", s.code().unwrap());
                }
            }
            Err(_e) => error!("{}", &_e.to_string()),
        }
    }
}
