mod arguments;
mod dir;
mod downloader;
mod error;
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
use simple_logger::SimpleLogger;
use steam::Steam;

#[tokio::main]
async fn main() -> Result<(), unv::Error> {
    SimpleLogger::new().init().unwrap();
    let args = Cli::parse();

    match &args.commands {
        Commands::Proton(proton) => {
            let steam = Steam::new()?;
            proton.run(steam).await?;
        }
        Commands::Gaming(gaming) => {
            let root = installer::root_command();
            let installer = installer::find_installer()?;
            let commands = gaming.commands(installer, root);
            run_commands(&commands)?;
        }
    }

    Ok(())
}
