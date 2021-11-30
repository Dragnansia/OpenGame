/// Print a simple log
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        println!("\x1b[0;37m[Log] {}\x1b[0m", format!($($arg)*))
    }}
}
pub(crate) use log;

/// Print error message
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        println!("\x1b[0;31m[Error] {}\x1b[0m", format!($($arg)*))
    }}
}
pub(crate) use error;

/// Print warning message
#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => {{
        println!("\x1b[0;33m[Warning] {}\x1b[0m", format!($($arg)*))
    }}
}
pub(crate) use warning;

/// Print a success message
#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => {{
        println!("\x1b[0;32m[Success] {}\x1b[0m", format!($($arg)*))
    }}
}
pub(crate) use success;

#[cfg(test)]
mod tests {
    #[test]
    fn log() {
        success!("I'm a success");
        log!("I'm a log");
        warning!("I'm a warning");
        error!("I'm a error");
    }
}
