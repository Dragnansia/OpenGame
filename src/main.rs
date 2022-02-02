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
use proton::update_protonge;
use steam::Steam;

#[tokio::main]
async fn main() -> Result<(), unv::Error> {
    let args = Cli::parse();

    match &args.commands {
        Commands::Proton(proton) => {
            let steam = Steam::new();
            if let Ok(steam) = steam {
                if let Some(v) = &proton.install {
                    proton::install_version(v, &steam).await?;
                }

                if proton.update {
                    update_protonge(&steam).await;
                }

                if let Some(p) = &proton.archive {
                    proton::install_archive_version(p, &steam);
                }

                if proton.list {
                    proton::list_version(&steam);
                }

                if let Some(v) = &proton.remove {
                    proton::remove_version(v, &steam)?;
                }

                if proton.clean {
                    proton::remove_cache()?;
                }
            } else {
                error!("Steam initialisation error: {}", steam.err().unwrap())
            }
        }
        Commands::Gaming(gaming) => match installer::find_installer() {
            Ok(cmds) => {
                let root = installer::root_command();
                let mut commands = vec![];

                if gaming.all {
                    commands.append(&mut cmds.all(&root));
                } else {
                    if gaming.gaming {
                        commands.append(&mut cmds.gaming(&root));
                    }

                    if gaming.lutris {
                        commands.append(&mut cmds.lutris(&root));
                    }

                    if gaming.heroic {
                        commands.append(&mut cmds.heroic_launcher(&root));
                    }

                    if gaming.overlay {
                        commands.append(&mut cmds.overlay(&root));
                    }

                    if gaming.replay_sorcery {
                        commands.append(&mut cmds.replay_sorcery(&root));
                    }

                    if gaming.mini_galaxy {
                        commands.append(&mut cmds.mini_galaxy(&root));
                    }
                }

                run_commands(&commands)?;
            }
            Err(err) => {
                error!("{}", err.to_string());
            }
        },
    }

    Ok(())
}
