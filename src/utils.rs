use crate::error;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

macro_rules! scan {
    ( $string:expr, $sep:expr, $( $x:ty ),+ ) => {{
        let mut iter = $string.split($sep);
        ($(iter.next().and_then(|word| word.parse::<$x>().ok()),)*)
    }}
}
pub(crate) use scan;

/// Return data information from `/etc/os-release` file
pub fn os_release_data(data: &str) -> Result<(String, String), error::unv::Error> {
    let file = File::open("/etc/os-release")?;

    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let (name, value) = scan!(line, "=", String, String);

        let name = name.ok_or(format!("No {} value", data))?;
        if name != data {
            continue;
        }

        let value = value.ok_or("Value is Empty")?.replace('\"', "");
        return Ok((name, value));
    }

    Err(format!("No found {} value", data).into())
}

pub fn user_validation(display: &str, response: fn(&str) -> bool) -> bool {
    print!("{}", display);
    io::stdout().flush().unwrap();

    let mut res = String::new();
    io::stdin().read_line(&mut res).unwrap_or_default();

    // Remove \n character at the end
    res.pop().unwrap_or_default();

    response(&res)
}
