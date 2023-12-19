#[derive(Debug)]
pub enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    pub fn from(color: &str) -> Option<Color> {
        match color.to_lowercase().as_str() {
            "red" => Some(Color::Red),
            "green" => Some(Color::Green),
            "blue" => Some(Color::Blue),
            _ => None,
        }
    }

    pub fn threshold(&self) -> i32 {
        match self {
            Color::Red => 12,
            Color::Green => 13,
            Color::Blue => 14,
        }
    }
}
