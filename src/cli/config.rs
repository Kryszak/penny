use clap::{Parser, ValueEnum};
use std::env::var;
use tui::style::Color;

#[derive(Clone, Debug, ValueEnum)]
pub enum ParsedColor {
    Cyan,
    Red,
    Magenta,
    Green,
    Blue,
}

impl ParsedColor {
    pub fn to_tui_color(&self) -> Color {
        match self {
            ParsedColor::Cyan => Color::Cyan,
            ParsedColor::Red => Color::Red,
            ParsedColor::Magenta => Color::Magenta,
            ParsedColor::Green => Color::Green,
            ParsedColor::Blue => Color::Blue,
        }
    }
}

/// Configuration parameters available to set as command line arguments
/// If not provided, defaults from struct are used
#[derive(Parser, Debug)]
#[command(about = "TUI Mp3 player")]
pub struct Config {
    /// First directory to open in file viewer after app start
    #[arg(
        long,
        default_value_t = var("HOME").unwrap_or(String::new()),
        help = "Starting directory to open for penny")]
    pub starting_directory: String,

    /// Open logs view and set log level to DEBUG
    #[arg(long, help = "Toggle logs with debug level")]
    pub debug: bool,

    #[arg(long, default_value_t = 64)]
    pub band_count: usize,

    #[arg(value_enum, long, default_value_t = ParsedColor::Cyan) ]
    pub color: ParsedColor,
}
