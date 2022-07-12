use crate::error::Error;

/// Settings for app
pub struct Settings {
    // Language

    // Launcher

    // Theme

    // What part enable ?

    // Paths (Cache, Temp, ...)
}

impl Settings {
    pub fn new() -> Result<Self, Error> {
        let _config_dir =
            dirs::config_dir().ok_or(Error::PathNotFound(String::from("~/.config")))?;

        Ok(Self {})
    }
}
