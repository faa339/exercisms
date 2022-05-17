// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    match level{
        LogLevel::Info    => return info(message),
        LogLevel::Warning => return warn(message),
        LogLevel::Error   => return error(message),
        LogLevel::Debug   => return debug(message),
    }
}
pub fn info(message: &str) -> String {
    return format!("[INFO]: {}", message);
}
pub fn warn(message: &str) -> String {
    return format!("[WARNING]: {}", message);
}
pub fn error(message: &str) -> String {
    return format!("[ERROR]: {}", message);
}

pub fn debug(message: &str)->String{
    return format!("[DEBUG]: {}", message);
}