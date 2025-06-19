use ratatui::style::Color;
use std::str::FromStr;

#[allow(dead_code)]
pub enum ColorScheme {
    Base,
    Blue,
    Crust,
    Flamingo,
    Green,
    Lavender,
    Mantle,
    Maroon,
    Mauve,
    Overlay0,
    Overlay1,
    Overlay2,
    Peach,
    Pink,
    Red,
    Rosewater,
    Sapphire,
    Sky,
    Subtext0,
    Subtext1,
    Surface0,
    Surface1,
    Surface2,
    Teal,
    Text,
    Yellow,
}

impl ColorScheme {
    pub fn v(&self) -> Color {
        match self {
            ColorScheme::Base => Color::from_str("#1e1e2e").unwrap(),
            ColorScheme::Blue => Color::from_str("#89b4fa").unwrap(),
            ColorScheme::Crust => Color::from_str("#11111b").unwrap(),
            ColorScheme::Flamingo => Color::from_str("#f2cdcd").unwrap(),
            ColorScheme::Green => Color::from_str("#a6e3a1").unwrap(),
            ColorScheme::Lavender => Color::from_str("#b4befe").unwrap(),
            ColorScheme::Mantle => Color::from_str("#181825").unwrap(),
            ColorScheme::Maroon => Color::from_str("#eba0ac").unwrap(),
            ColorScheme::Mauve => Color::from_str("#cba6f7").unwrap(),
            ColorScheme::Overlay0 => Color::from_str("#6c7086").unwrap(),
            ColorScheme::Overlay1 => Color::from_str("#7f849c").unwrap(),
            ColorScheme::Overlay2 => Color::from_str("#9399b2").unwrap(),
            ColorScheme::Peach => Color::from_str("#fab387").unwrap(),
            ColorScheme::Pink => Color::from_str("#f5c2e7").unwrap(),
            ColorScheme::Red => Color::from_str("#f38ba8").unwrap(),
            ColorScheme::Rosewater => Color::from_str("#f5e0dc").unwrap(),
            ColorScheme::Sapphire => Color::from_str("#74c7ec").unwrap(),
            ColorScheme::Sky => Color::from_str("#89dceb").unwrap(),
            ColorScheme::Subtext0 => Color::from_str("#a6adc8").unwrap(),
            ColorScheme::Subtext1 => Color::from_str("#bac2de").unwrap(),
            ColorScheme::Surface0 => Color::from_str("#313244").unwrap(),
            ColorScheme::Surface1 => Color::from_str("#45475a").unwrap(),
            ColorScheme::Surface2 => Color::from_str("#585b70").unwrap(),
            ColorScheme::Teal => Color::from_str("#94e2d5").unwrap(),
            ColorScheme::Text => Color::from_str("#cdd6f4").unwrap(),
            ColorScheme::Yellow => Color::from_str("#f9e2af").unwrap(),
        }
    }
}
