mod arch;
mod fedora;
pub mod installer;
mod ubuntu;

use crate::{error::unv, timer, utils::user_validation};
use log::{error, info, warn};
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
    if cmds.is_empty() {
        warn!("No command to run");
        return Ok(());
    }

    let mto = if cmds.len() > 1 { "s" } else { "" };
    // Before running commands, we need to get user agreement
    info!("{} command{} to run:", cmds.len(), mto);
    cmds.iter().for_each(|cmd| println!("  - {}", cmd));

    let qst = format!("Run command{} ? [Y/n]: ", mto);
    if user_validation(&qst, |r| {
        r == "n" || r == "N" || r != "y" && r != "Y" && r != ""
    }) {
        return Ok(());
    }

    for command in cmds {
        let timer = timer::current_time();
        let mut args: Vec<&str> = command.split(' ').collect();
        let cmd = args.remove(0);

        let exit_status = Command::new(cmd).args(&args).status()?;
        if exit_status.success() {
            let time = timer::get_duration(&timer);
            info!(
                "No error with the command ({} sec{})",
                time,
                if time > 1 { "s" } else { "" }
            );
        } else {
            error!("Command error code: {:?}", exit_status.code());
        }
    }

    Ok(())
}
