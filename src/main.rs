use clap::{App, Arg, ArgMatches, SubCommand};

mod net;
mod proton;
mod steam;

fn matches_argument() -> ArgMatches<'static> {
    App::new("og")
        .version("0.0.1")
        .author("RomualdAuc")
        .about("A simple program to install gaming dependencies on linux computer")
        .subcommand(
            SubCommand::with_name("proton")
                .about("ProtonGE gestion (Install / Remove / List)")
                .arg(
                    Arg::with_name("install")
                        .short("i")
                        .long("install")
                        .help("Install a specific version of ProtonGE")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("list")
                        .short("l")
                        .long("list")
                        .help("Print all version of proton install")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("remove")
                        .short("r")
                        .long("remove")
                        .help("Remove a specific version")
                        .takes_value(true),
                ),
        )
        .get_matches()
}

fn main() {
    let steam = steam::init_steam_data();
    let matches = matches_argument();

    if let Some(matches) = matches.subcommand_matches("proton") {
        if matches.is_present("install") {
            proton::install_version(matches.value_of("install").unwrap(), &steam);
        }

        if matches.is_present("remove") {
            proton::remove_version(matches.value_of("remove").unwrap(), &steam);
        }

        if matches.is_present("list") {
            proton::list_version(&steam);
        }
    }
}
