//! Used to install package from package manager
use crate::{error, os::release_data};
use serde::Deserialize;

/// All function to install gaming dependencies or package
/// on distro
#[derive(Debug, Deserialize)]
pub struct Installer {
    pub distro_name: String,
}

impl Installer {
    /// Create a new Installer for package
    ///
    /// Not found current distro name is a possibility
    pub fn new() -> Result<Self, error::Error> {
        Ok(Self {
            distro_name: Self::found_disto_name()?,
        })
    }

    /// Return all commands necessary to install package
    /// and format to used package manager to install
    ///
    /// Need to used with other function to run all commands
    ///
    /// # Exemple
    /// ```
    /// use core::package::installer::Installer;
    ///
    /// let commands = Installer::new().unwrap().get_commands("steam");
    /// ```
    pub fn get_commands(&self, _group_name: &str) -> Vec<String> {
        todo!()
    }

    /// Return current distro name
    fn found_disto_name() -> Result<String, error::Error> {
        release_data("NAME")
    }
}
