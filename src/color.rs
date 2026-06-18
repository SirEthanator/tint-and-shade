use crate::constants::*;
use crate::cursor::fmt_move_cursor_by;

pub struct Color {
    pub hex: String,
    pub rgb: [u8; 3],
    pub title: String,
}

impl Color {
    pub fn from_hex(hex: &str, title: &str) -> Self {
        let rgb = Color::hex_to_rgb(hex);

        Color {
            hex: String::from(hex),
            rgb,
            title: String::from(title),
        }
    }

    pub fn from_rgb(rgb: &[u8; 3], title: &str) -> Self {
        let hex = Color::rgb_to_hex(rgb);
        Color {
            hex,
            rgb: *rgb,
            title: String::from(title),
        }
    }

    fn hex_to_rgb(hex: &str) -> [u8; 3] {
        let mut result: [u8; 3] = [0, 0, 0];
        for i in 0..hex.len() / 2 {
            result[i] = u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).unwrap();
        }
        result
    }

    fn rgb_to_hex(rgb: &[u8; 3]) -> String {
        let mut result = String::new();
        for c in rgb {
            let hex = format!("{:02X}", c);
            result += &hex;
        }
        result
    }

    pub fn parse_hex(hex_str: &str) -> Option<String> {
        let mut hex = hex_str;

        if hex.starts_with("#") {
            let mut chars = hex.chars();
            chars.next();
            hex = chars.as_str();
        }

        if hex.len() != 6 || !hex.chars().all(|c| c.is_ascii_hexdigit()) {
            return None;
        }

        Some(String::from(hex))
    }

    pub fn parse_rgb(rgb_str: &str) -> Option<[u8; 3]> {
        let rgb_str = rgb_str.trim();

        if !rgb_str.to_lowercase().starts_with("rgb(") || !rgb_str.ends_with(")") {
            return None;
        }

        let stripped = &rgb_str[4..rgb_str.len() - 1];
        let mut parts = stripped.split(',');

        let r = parts.next()?.trim().parse::<u8>().ok()?;
        let g = parts.next()?.trim().parse::<u8>().ok()?;
        let b = parts.next()?.trim().parse::<u8>().ok()?;

        if parts.next().is_some() {
            return None;
        }

        Some([r, g, b])
    }

    pub fn rgb_string(&self) -> String {
        format!(
            "rgb({:03}, {:03}, {:03})",
            self.rgb[0], self.rgb[1], self.rgb[2]
        )
    }

    pub fn hex_string(&self) -> String {
        format!("#{}", &self.hex)
    }

    pub fn shade(&self, percentage: u8) -> Self {
        let factor: f64 = 1.0 - percentage as f64 / 100.0;
        let mut out_rgb: [u8; 3] = [0, 0, 0];

        for (i, &val) in self.rgb.iter().enumerate() {
            out_rgb[i] = (factor * val as f64).round() as u8;
        }

        Self::from_rgb(&out_rgb, "Shade")
    }

    pub fn tint(&self, percentage: u8) -> Self {
        let factor: f64 = percentage as f64 / 100.0;
        let mut out_rgb: [u8; 3] = [0, 0, 0];

        for (i, &val) in self.rgb.iter().enumerate() {
            out_rgb[i] = (val as f64 + ((255.0 - val as f64) * factor)).round() as u8;
        }

        Self::from_rgb(&out_rgb, "Tint")
    }

    pub fn fmt_box(&self, x: usize) -> String {
        let title_len = self.title.len();
        let mut hex = self.hex_string();
        let mut rgb_str = self.rgb_string();
        let mut title = format!("{}{}{}", BOLD, self.title, UNBOLD);

        let side_padding = " ".repeat(BOX_SIDE_PADDING);

        rgb_str = format!(
            "{}{}{}",
            &side_padding,
            rgb_str,
            " ".repeat(BOX_WIDTH - rgb_str.len() - BOX_SIDE_PADDING)
        );
        title = format!(
            "{}{}{}",
            &side_padding,
            title,
            " ".repeat(BOX_WIDTH - title_len - BOX_SIDE_PADDING)
        );
        hex = format!(
            "{}{}{}",
            &side_padding,
            hex,
            " ".repeat(BOX_WIDTH - hex.len() - BOX_SIDE_PADDING)
        );

        let top_bottom_padding = " ".repeat(BOX_WIDTH);

        let lines = [
            top_bottom_padding.clone(),
            title,
            rgb_str,
            hex,
            top_bottom_padding,
        ];
        let line_separator = format!("\n{}", fmt_move_cursor_by(x as i16, 0));
        let out = lines.join(&line_separator);

        Color::highlight_string(&out, &self.rgb)
    }

    fn get_text_color(bg: &[u8; 3]) -> &'static str {
        // https://www.w3.org/TR/AERT/#color-contrast
        let brightness: f64 =
            ((bg[0] as f64 * 299.0) + (bg[1] as f64 * 587.0) + (bg[2] as f64 * 114.0)) / 1000.0;
        if brightness > 125.0 {
            TRUE_BLACK
        } else {
            TRUE_WHITE
        }
    }

    fn highlight_string(str: &str, bg: &[u8; 3]) -> String {
        let bg_str = format!("\x1b[48;2;{};{};{}m", bg[0], bg[1], bg[2]);
        let fg_str = Color::get_text_color(bg);
        format!("{}{}{}{}", bg_str, fg_str, str, RESET)
    }
}
