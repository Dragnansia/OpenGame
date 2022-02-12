mod arch;
mod fedora;
pub mod installer;
mod ubuntu;

use crate::{error::unv, timer};
use log::{error, info};
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
pub fn run_commands(cmds: &Vec<String>) -> Result<(), unv::Error> {
    for command in cmds {
        let timer = timer::current_time();
        let mut args: Vec<&str> = command.split(' ').collect();
        let cmd = args[0];
        args.remove(0);

        let exit_status = Command::new(cmd).args(&args).status()?;
        if exit_status.success() {
            info!(
                "No error on last command ({} sec(s))",
                timer::get_duration(&timer)
            );
        } else {
            error!("Command error code: {:?}", exit_status.code());
        }
    }

    Ok(())
}
