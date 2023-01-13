use clap::Parser;
use std::env::var;

#[derive(Parser, Debug)]
#[command(about = "TUI Mp3 player")]
pub struct Config {
    #[arg(
        long,
        default_value_t = var("HOME").unwrap_or(String::new()),
        help = "Starting directory to open for penny")]
    pub starting_directory: String,

    #[arg(long, help = "Toggle logs with debug level")]
    pub debug: bool,
}
