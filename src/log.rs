pub const RESET: &str = "\x1b[0m";
pub const WARN_COLOR: &str = "\x1b[33m";
pub const ERROR_COLOR: &str = "\x1b[31m";

pub fn warn(msg: &str) {
    eprintln!("{}Warning:{} {}", WARN_COLOR, RESET, msg);
}

pub fn error(msg: &str) {
    eprintln!("{}Error:{} {}", ERROR_COLOR, RESET, msg);
}
