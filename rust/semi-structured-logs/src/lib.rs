// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Debug,
    Error,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    match level {
        LogLevel::Info => info(message),
        LogLevel::Warning => warn(message),
        LogLevel::Debug => debug(message),
        LogLevel::Error => error(message),
    }
}
pub fn info(message: &str) -> String {
    let mut info_message = "[INFO]: ".to_owned();
    info_message.push_str(message);
    return info_message.to_string();
}
pub fn warn(message: &str) -> String {
    let mut warning_message = "[WARNING]: ".to_owned();
    warning_message.push_str(message);
    return warning_message.to_string();
}
pub fn error(message: &str) -> String {
    let mut error_message = "[ERROR]: ".to_owned();
    error_message.push_str(message);
    return error_message.to_string();
}
pub fn debug(message: &str) -> String {
    let mut debug_message = "[DEBUG]: ".to_owned();
    debug_message.push_str(message);
    return debug_message.to_string();
}
