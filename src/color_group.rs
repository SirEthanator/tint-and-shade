use crate::color::Color;
use crate::constants::*;
use crate::cursor::move_cursor_by;
use std::io::{self, Write};

pub struct ColorGroup {
    pub original: Color,
    pub shaded: Color,
    pub tinted: Color,
}

impl ColorGroup {
    pub fn print_colors(&self, term_width: usize) {
        let colors = [&self.tinted, &self.original, &self.shaded];

        let mut cursor_x = 0;

        for (i, color) in colors.iter().enumerate() {
            print!("{}", color.fmt_box(cursor_x));
            let _ = io::stdout().flush();

            // Stop early on last box, nothing more needs to be done
            if i == colors.len() - 1 {
                break;
            }

            let dx = BOX_WIDTH + BOX_SPACING;
            let new_cursor_x = cursor_x + dx;

            if new_cursor_x + BOX_WIDTH > term_width {
                print!("\n\n");
                cursor_x = 0;
            } else {
                move_cursor_by(BOX_SPACING as i16, -(BOX_HEIGHT as i16) + 1);
                cursor_x = new_cursor_x;
            }
        }

        println!();
    }
}
