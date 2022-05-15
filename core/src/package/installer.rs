//! Used to install package from package manager
use serde::Deserialize;
use std::collections::HashMap;

/// All function to install package on distro
/// are saved on HashMap
#[derive(Debug, Deserialize)]
pub struct Installer {
    pub distro_name: String,

    /// Package Name - Commands
    pub packages: HashMap<String, Vec<String>>,
}

impl Installer {
    fn new() -> Self {
        Self {
            distro_name: String::new(),
            packages: HashMap::new(),
        }
    }
}
