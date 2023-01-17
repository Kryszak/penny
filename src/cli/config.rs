use clap::Parser;
use std::env::var;

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
}
