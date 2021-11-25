use colored::*;

/// TODO: Find a better way for log somthing like println
/// macro and no just a call of this for removr AsRef<str>
/// and format

/// Print a simple log
pub fn log<T: AsRef<str>>(message: T) {
    println!("{} {}", "[Log]".white(), message.as_ref().white());
}

/// Print error message
pub fn error<T: AsRef<str>>(message: T) {
    println!("{} {}", "[Error]".red(), message.as_ref().red());
}

/// Print warning message
pub fn warning<T: AsRef<str>>(message: T) {
    println!("{} {}", "[Warning]".yellow(), message.as_ref().yellow());
}

/// Print a success message
pub fn success<T: AsRef<str>>(message: T) {
    println!("{} {}", "[Success]".green(), message.as_ref().green());
}
