//! Used to find current installed package manager on
//! current distro, and call basic command
//!
//! Can be used to get package installed on current distro
use std::{io, path::Path};

static PACKAGES_MANAGER: [&str; 4] = ["dnf", "pacman", "apt", "yum"];

/// Return current
pub struct PackageManager {
    name: &'static str,
}

impl PackageManager {
    /// Find current package manager installed
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            name: Self::found_package_manager()?,
        })
    }

    /// Return current installed package manager
    /// found on the system
    fn found_package_manager() -> io::Result<&'static str> {
        if let Some(pm) = PACKAGES_MANAGER
            .into_iter()
            .find(|&pm| Path::new(&format!("/usr/bin/{pm}")).exists())
        {
            return Ok(pm);
        }

        Err(io::Error::new(io::ErrorKind::NotFound, "Package manager"))
    }

    /// Install all packages
    pub fn install_packages<T: AsRef<str>>(&self, commands: &[T]) {
        todo!()
    }
}
