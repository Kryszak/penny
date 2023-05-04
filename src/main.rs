use application::App;
use clap::Parser;
use cli::config::Config;
use runner::run_app;
use std::io;

mod application;
mod cli;
mod external;
mod files;
mod input;
mod player;
mod runner;

fn main() -> io::Result<()> {
    let config = Config::parse();
    match App::new(&config) {
        Some(mut app) => {
            run_app(&mut app)?;
        }
        None => {
            println!("Failed to open {}, terminating.", config.starting_directory)
        }
    };

    Ok(())
}
