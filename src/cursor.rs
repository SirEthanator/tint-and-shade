use std::io::{self, Write};

pub fn fmt_move_cursor_by(dx: i16, dy: i16) -> String {
    let mut output = String::new();

    if dy > 0 {
        output += &format!("\x1b[{}B", dy);
    } else if dy < 0 {
        output += &format!("\x1b[{}A", dy.abs());
    }

    if dx > 0 {
        output += &format!("\x1b[{}C", dx);
    } else if dx < 0 {
        output += &format!("\x1b[{}D", dx.abs());
    }

    output
}

pub fn move_cursor_by(dx: i16, dy: i16) {
    print!("{}", fmt_move_cursor_by(dx, dy));
    let _ = io::stdout().flush();
}
