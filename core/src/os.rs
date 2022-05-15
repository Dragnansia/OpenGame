//! All functions used for os data and interaction

use crate::utils::scan;
use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind},
};

/// Return data information from `/etc/os-release` by name of the data
/// and if not found return error
pub fn release_data(data_name: &str) -> Result<String, Error> {
    let file = File::open("/etc/os-release")?;

    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let (name, value) = scan!(line, "=", String, String);

        let name = name.unwrap_or_default();
        if name != data_name {
            continue;
        }

        let value = value.unwrap_or_default().replace('\"', "");
        return Ok(value);
    }

    Err(Error::new(ErrorKind::NotFound, ""))
}
