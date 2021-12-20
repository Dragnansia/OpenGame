#![feature(allow_internal_unstable)]

mod dir;
mod downloader;
mod log;
mod timer;
mod pckg;
mod proton;
mod steam;

use clap::{App, Arg, ArgMatches, SubCommand};
use pckg::{installer, run_commands};
use steam::Steam;

fn matches_argument() -> ArgMatches<'static> {
    App::new("og")
        .version("0.0.1")
        .about("A simple program to install gaming dependencies on Linux computer")
        .subcommand(
            SubCommand::with_name("proton")
                .about("ProtonGE management (Install / Remove / List)")
                .arg(
                    Arg::with_name("install")
                        .short("i")
                        .long("install")
                        .help("Install a specific version of ProtonGE")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("update")
                        .short("u")
                        .long("update")
                        .help("Install latest ProtonGE version")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("archive")
                        .short("a")
                        .long("archive")
                        .help("Install archive with the path")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("list")
                        .short("l")
                        .long("list")
                        .help("Print all versions of Proton installs")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("remove")
                        .short("r")
                        .long("remove")
                        .help("Remove a specific version")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("clean")
                        .short("c")
                        .long("clean")
                        .help("Remove the cache directory of ProtonGE")
                        .takes_value(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("gaming")
                .about(
                    "Install gaming dependencies, install all dependencies if no FLAGS are added",
                )
                .arg(
                    Arg::with_name("gaming")
                        .short("g")
                        .long("gaming")
                        .help("Install gaming dependencies")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("lutris")
                        .short("l")
                        .long("lutris")
                        .help("Install Lutris")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("heroic")
                        .short("h")
                        .long("heroic")
                        .help("Install Heroic Launcher")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("overlay")
                        .short("o")
                        .long("overlay")
                        .help("Install Overlay")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("replay-sorcery")
                        .short("r")
                        .long("replay-sorcery")
                        .help("Install ReplaySorcery")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("minigalaxy")
                        .short("m")
                        .long("minigalaxy")
                        .help("Install MiniGalaxy")
                        .takes_value(false),
                ),
        )
        .get_matches()
}

#[tokio::main]
async fn main() {
    let matches = matches_argument();

    if let Some(matches) = matches.subcommand_matches("proton") {
        match Steam::new() {
            Ok(steam) => {
                if matches.is_present("install") {
                    proton::install_version(matches.value_of("install").unwrap(), &steam).await;
                }

                if matches.is_present("remove") {
                    proton::remove_version(matches.value_of("remove").unwrap(), &steam);
                }

                if matches.is_present("list") {
                    proton::list_version(&steam);
                }

                if matches.is_present("archive") {
                    proton::install_archive_version(matches.value_of("archive").unwrap(), &steam);
                }

                if matches.is_present("update") {
                    proton::update_protonge(&steam).await;
                }

                if matches.is_present("clean") {
                    proton::remove_cache();
                }
            }
            Err(e) => error!("Steam initialisation error: {}", e),
        }
    }

    if let Some(matches) = matches.subcommand_matches("gaming") {
        let root = installer::root_command();
        match installer::find_installer() {
            Ok(commands) => {
                if matches.args.len() == 0 {
                    run_commands(&commands.all(&root).await);
                } else {
                    if matches.is_present("gaming") {
                        run_commands(&commands.gaming(&root).await);
                    }

                    if matches.is_present("lutris") {
                        run_commands(&commands.lutris(&root).await);
                    }

                    if matches.is_present("heroic") {
                        run_commands(&commands.heroic_launcher(&root).await);
                    }

                    if matches.is_present("overlay") {
                        run_commands(&commands.overlay(&root).await);
                    }

                    if matches.is_present("replay-sorcery") {
                        run_commands(&commands.replay_sorcery(&root).await);
                    }

                    if matches.is_present("minigalaxy") {
                        run_commands(&commands.mini_galaxy(&root).await);
                    }
                }
            }
            Err(err) => error!("{}", err.to_string()),
        }
    }
}
