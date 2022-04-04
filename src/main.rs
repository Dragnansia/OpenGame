mod arguments;
mod dir;
mod error;
mod pckg;
mod proton;
mod timer;
mod utils;

use arguments::{Cli, Commands};
use clap::StructOpt;
use error::unv;
use lamodin::launcher::steam::Steam;
use log::info;
use pckg::{installer, run_commands};

#[tokio::main]
async fn main() -> Result<(), unv::Error> {
    color_logger::init()?;

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
