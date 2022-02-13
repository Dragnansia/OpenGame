mod arguments;
mod dir;
mod downloader;
mod error;
mod pckg;
mod proton;
mod steam;
mod timer;
mod utils;

use arguments::{Cli, Commands};
use clap::StructOpt;
use error::unv;
use log::info;
use pckg::{
    installer::{self},
    run_commands,
};
use steam::Steam;

#[tokio::main]
async fn main() -> Result<(), unv::Error> {
    simple_logger::init_with_level(log::Level::Info)?;
    let args = Cli::parse();

    match &args.commands {
        Commands::Proton(proton) => {
            let steam = Steam::new()?;
            proton.run(steam).await?;
        }
        Commands::Gaming(gaming) => {
            let root = installer::root_command();
            info!("Root command => {}", root);

            let (distro_name, installer) = installer::find_installer()?;
            info!("Distro name => {}", distro_name);

            let commands = gaming.commands(installer, root);
            run_commands(&commands)?;
        }
    }

    Ok(())
}
