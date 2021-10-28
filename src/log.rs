use colored::*;

pub fn log(message: &str) {
    let arrow = "[Log]".white();
    println!("{} {}", arrow, message.white());
}

pub fn error(message: &str) {
    let arrow = "[Error]".red();
    println!("{} {}", arrow, message.red());
}

pub fn warning(message: &str) {
    let arrow = "[Warning]".yellow();
    println!("{} {}", arrow, message.yellow());
}
