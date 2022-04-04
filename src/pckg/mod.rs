mod arch;
mod fedora;
pub mod installer;
mod ubuntu;

use crate::{error::unv, timer};
use log::{error, info, warn};
use std::{
    io::{self, Write},
    process::Command,
};

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
    print!("Run command{} ? [Y/n]: ", mto);
    // Need this to display print! macro
    io::stdout().flush()?;

    let mut run_commands = String::new();
    io::stdin().read_line(&mut run_commands)?;

    let rcmd = run_commands.get(..1).ok_or("")?;
    if rcmd == "n" || rcmd == "N" || rcmd != "y" && rcmd != "Y" && rcmd != "\n" {
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
