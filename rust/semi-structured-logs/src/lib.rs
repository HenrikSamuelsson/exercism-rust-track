// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    unimplemented!()
}
pub fn info(message: &str) -> String {
    let mut info_message = "[INFO]: ".to_owned();
    info_message.push_str(message);
    return info_message.to_string();
}
pub fn warn(message: &str) -> String {
    unimplemented!()
}
pub fn error(message: &str) -> String {
    unimplemented!()
}
