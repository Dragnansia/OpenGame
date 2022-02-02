/// Print a simple white message
///
/// # Examples
/// ```
/// log!("hello there!");
/// log!("format {} arguments", "some");
/// ```
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        println!("\x1b[0;37m[Log] {}\x1b[0m", format!($($arg)*))
    }}
}
pub(crate) use log;

/// Print a simple red message
///
/// # Examples
/// ```
/// error!("hello there!");
/// error!("format {} arguments", "some");
/// ```
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        println!("\x1b[0;31m[Error] {}\x1b[0m", format!($($arg)*))
    }}
}
pub(crate) use error;

/// Print a simple yellow message
///
/// # Examples
/// ```
/// warning!("hello there!");
/// warning!("format {} arguments", "some");
/// ```
#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => {{
        println!("\x1b[0;33m[Warning] {}\x1b[0m", format!($($arg)*))
    }}
}
pub(crate) use warning;

/// Print a simple green message
///
/// # Examples
/// ```
/// success!("hello there!");
/// success!("format {} arguments", "some");
/// ```
#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => {{
        println!("\x1b[0;32m[Success] {}\x1b[0m", format!($($arg)*))
    }}
}
pub(crate) use success;
