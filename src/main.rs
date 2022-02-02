mod arguments;
mod dir;
mod downloader;
mod error;
mod log;
mod pckg;
mod proton;
mod steam;
mod timer;

use arguments::{Cli, Commands};
use clap::StructOpt;
use error::unv;
use pckg::{
    installer::{self},
    run_commands,
};
use steam::Steam;

#[tokio::main]
async fn main() -> Result<(), unv::Error> {
    let args = Cli::parse();

    match &args.commands {
        Commands::Proton(proton) => {
            let steam = Steam::new();
            if let Ok(steam) = steam {
                proton.run(steam).await?;
            } else {
                error!("Steam initialisation error: {}", steam.err().unwrap())
            }
        }
        Commands::Gaming(gaming) => match installer::find_installer() {
            Ok(installer) => {
                let root = installer::root_command();
                let commands = gaming.commands(installer, root);
                run_commands(&commands)?;
            }
            Err(err) => {
                error!("{}", err.to_string());
            }
        },
    }

    Ok(())
}
