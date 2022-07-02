//! All function useful for command
use crate::error;
use std::{collections::VecDeque, process::Command};

/// Run all commands
pub fn run_commands(commands: &[&str]) -> Result<(), error::Error> {
    for command in commands {
        let mut args: VecDeque<&str> = command.split(' ').collect();
        let scommand = Command::new(
            args.pop_front()
                .ok_or(error::Error::CommandError(command.to_string()))?,
        )
        .args(args)
        .spawn();

        if let Err(_err) = scommand {
            // Add log system
        }
    }

    Ok(())
}
