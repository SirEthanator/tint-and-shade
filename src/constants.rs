pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";
pub const UNBOLD: &str = "\x1b[22m";
pub const TRUE_BLACK: &str = "\x1b[38;2;0;0;0m";
pub const TRUE_WHITE: &str = "\x1b[38;2;255;255;255m";

pub const BOX_SIDE_PADDING: usize = 2;
pub const BOX_SPACING: usize = 0;

pub const BOX_WIDTH: usize = "rgb(000, 000, 000)".len() + BOX_SIDE_PADDING * 2;

// Do not change unless fmt_box is changed
// This should hold the number of lines that the box uses
pub const BOX_HEIGHT: usize = 5;
