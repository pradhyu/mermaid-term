pub struct Ansi;

impl Ansi {
    pub const RESET: &'static str = "\x1b[0m";
    pub const BOLD: &'static str = "\x1b[1m";
    pub const DIM: &'static str = "\x1b[2m";

    // Standard colors
    pub const RED: &'static str = "\x1b[31m";
    pub const GREEN: &'static str = "\x1b[32m";
    pub const YELLOW: &'static str = "\x1b[33m";
    pub const BLUE: &'static str = "\x1b[34m";
    pub const MAGENTA: &'static str = "\x1b[35m";
    pub const CYAN: &'static str = "\x1b[36m";
    pub const WHITE: &'static str = "\x1b[37m";

    // Bright colors
    pub const BRIGHT_BLUE: &'static str = "\x1b[94m";
    pub const BRIGHT_CYAN: &'static str = "\x1b[96m";
    pub const BRIGHT_GREEN: &'static str = "\x1b[92m";
    pub const BRIGHT_YELLOW: &'static str = "\x1b[93m";
    pub const BRIGHT_MAGENTA: &'static str = "\x1b[95m";

    pub fn rgb(r: u8, g: u8, b: u8) -> String {
        format!("\x1b[38;2;{};{};{}m", r, g, b)
    }

    pub fn parse_color(c: &str) -> String {
        match c.to_lowercase().as_str() {
            "red" => Self::RED.to_string(),
            "green" => Self::GREEN.to_string(),
            "yellow" | "orange" => Self::YELLOW.to_string(),
            "blue" => Self::BLUE.to_string(),
            "magenta" | "purple" => Self::MAGENTA.to_string(),
            "cyan" => Self::CYAN.to_string(),
            "white" => Self::WHITE.to_string(),
            s if s.starts_with('#') && s.len() == 7 => {
                if let (Ok(r), Ok(g), Ok(b)) = (
                    u8::from_str_radix(&s[1..3], 16),
                    u8::from_str_radix(&s[3..5], 16),
                    u8::from_str_radix(&s[5..7], 16),
                ) {
                    Self::rgb(r, g, b)
                } else {
                    Self::CYAN.to_string()
                }
            }
            _ => Self::CYAN.to_string(),
        }
    }
}
