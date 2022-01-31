use clap::{AppSettings, Args, Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
pub struct Cli {
    #[clap(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// ProtonGE management (Install / Remove / List)
    Proton(Proton),

    /// Install gaming dependencies, install all dependencies if no FLAGS are added
    Gaming(Gaming),
}

#[derive(Args)]
pub struct Proton {
    #[clap(short, long)]
    /// Install a specific version of ProtonGE
    pub install: Option<String>,

    #[clap(short, long)]
    /// Install latest ProtonGE version if not already install
    pub update: bool,

    #[clap(short, long)]
    /// Install archive with the path
    pub archive: Option<String>,

    #[clap(short, long)]
    /// Print all versions of Proton installs
    pub list: bool,

    #[clap(short, long)]
    /// Remove a specific version
    pub remove: Option<String>,

    #[clap(short, long)]
    /// Remove the cache directory of ProtonGE
    pub clean: bool,
}

#[derive(Args)]
pub struct Gaming {
    #[clap(short, long)]
    /// Install all dependencies
    pub all: bool,

    #[clap(short, long)]
    /// Install gaming dependencies
    pub gaming: bool,

    #[clap(short, long)]
    /// Install Lutris
    pub lutris: bool,

    #[clap(short = 'H', long)]
    /// Install Heroic Launcher
    pub heroic: bool,

    #[clap(short, long)]
    /// Install Overlay
    pub overlay: bool,

    #[clap(short, long)]
    /// Install ReplaySorcery
    pub replay_sorcery: bool,

    #[clap(short, long)]
    /// Install MiniGalaxy
    pub mini_galaxy: bool,
}
