use crate::error;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

macro_rules! scan {
    ( $string:expr, $sep:expr, $( $x:ty ),+ ) => {{
        let mut iter = $string.split($sep);
        ($(iter.next().and_then(|word| word.parse::<$x>().ok()),)*)
    }}
}

pub(crate) use scan;

pub fn os_release_data(data: &str) -> Result<(String, String), error::unv::Error> {
    let file = File::open("/etc/os-release")?;

    let reader = BufReader::new(file);
    for (_, line) in reader.lines().enumerate() {
        let line = line?;

        let (name, value) = scan!(line, "=", String, String);

        let name = name.ok_or("No NAME value")?;
        if name != data {
            continue;
        }

        let value = value.ok_or("Value is Empty")?.replace('\"', "");
        return Ok((name, value));
    }

    Err(format!("No found {} value", data).into())
}
