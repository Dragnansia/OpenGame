use crate::{
    error::unv,
    pckg::installer::Installer,
    proton::{self, ProtonDownload},
};
use clap::{Args, Parser, Subcommand};
use lamodin::{launcher::steam::Steam, modifier::ModifierImpl};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
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

impl Proton {
    pub async fn run(&self, steam: Steam) -> Result<(), unv::Error> {
        if let Some(v) = &self.install {
            let versions = Steam::versions().await?;
            let release = versions
                .iter()
                .find(|pe| pe.tag_name.contains(v))
                .ok_or("err")?;

            let asset = release
                .assets
                .iter()
                .find(|a| a.name.ends_with("tar.gz"))
                .ok_or("")?;

            steam
                .install(asset, ProtonDownload::new(&release.name))
                .await?;
        }

        if self.update {
            proton::update_protonge(&steam).await?;
        }

        if let Some(p) = &self.archive {
            proton::install_archive_version(p, &steam)?;
        }

        if self.list {
            proton::list_version(&steam);
        }

        if let Some(v) = &self.remove {
            proton::remove_version(v, &steam)?;
        }

        if self.clean {
            proton::remove_cache()?;
        }

        Ok(())
    }
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

impl Gaming {
    pub fn commands(&self, installer: &dyn Installer, root: String) -> Vec<String> {
        let mut commands = vec![];

        if self.all {
            commands.append(&mut installer.all(&root));
        } else {
            if self.gaming {
                commands.append(&mut installer.gaming(&root));
            }

            if self.lutris {
                commands.append(&mut installer.lutris(&root));
            }

            if self.heroic {
                commands.append(&mut installer.heroic_launcher(&root));
            }

            if self.overlay {
                commands.append(&mut installer.overlay(&root));
            }

            if self.replay_sorcery {
                commands.append(&mut installer.replay_sorcery(&root));
            }

            if self.mini_galaxy {
                commands.append(&mut installer.mini_galaxy(&root));
            }
        }

        commands
    }
}
