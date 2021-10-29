use colored::*;

/// Print a simple log
pub fn log(message: &str) {
    println!("{} {}", "[Log]".white(), message.white());
}

/// Print error message
pub fn error(message: &str) {
    println!("{} {}", "[Error]".red(), message.red());
}

/// Print warning message
pub fn warning(message: &str) {
    println!("{} {}", "[Warning]".yellow(), message.yellow());
}

/// Print a success message
pub fn success(message: &str) {
    println!("{} {}", "[Success]".green(), message.green());
}
