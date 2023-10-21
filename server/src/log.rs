use chrono::prelude::*;
use colored::{Colorize, ColoredString};

#[allow(dead_code)]
pub enum LogLevel {
    Stack,
    Debug,
    Verbose,
    Info,
    Warn,
    Error,
    None,
}

impl LogLevel {
    fn get_level_str(&self) -> ColoredString {
        match self {
            LogLevel::Stack => "STACK".normal().bold(),
            LogLevel::Debug => "DEBUG".green().bold(),
            LogLevel::Verbose => "VERBO".normal().bold(),
            LogLevel::Info => " INFO".bold(),
            LogLevel::Warn => " WARN".yellow().bold(),
            LogLevel::Error => "ERROR".red().bold(),
            LogLevel::None => "".normal().bold(),
        }
    }
}

pub fn log(level: LogLevel, msg: &str) {
    let local_time = Local::now()
        .format("%d-%m-%Y %H:%M:%S");
    let level_str: ColoredString = level.get_level_str();

    // Implement thread logging (get the tid and stuff ykyk)

    println!("{}|\t{}|{}\n", local_time, level_str, msg);

    // Grab mutex lock for file to print to file
}

#[allow(dead_code)]
pub fn info(msg: &str) {
    log(LogLevel::Info, msg);
}

#[allow(dead_code)]
pub fn warn(msg: &str) {
    log(LogLevel::Warn, msg);
}

#[allow(dead_code)]
pub fn error(msg: &str) {
    log(LogLevel::Error, msg);
}

#[allow(dead_code)]
pub fn debug(msg: &str) {
    log(LogLevel::Debug, msg);
}