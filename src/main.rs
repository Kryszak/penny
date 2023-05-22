use application::App;
use clap::Parser;
use cli::config::Config;
use input::EventBus;
use runner::run_app;
use std::{
    io,
    sync::{Arc, Mutex},
    time::Duration,
};

mod application;
mod cli;
mod external;
mod files;
mod input;
mod player;
mod runner;
mod queue;

fn main() -> io::Result<()> {
    let config = Config::parse();
    let tick_rate = Duration::from_millis(150);
    let events = Arc::new(Mutex::new(EventBus::new(tick_rate)));
    match App::new(&config, events.clone()) {
        Some(mut app) => {
            run_app(&mut app, events)?;
        }
        None => {
            println!("Failed to open {}, terminating.", config.starting_directory)
        }
    };

    Ok(())
}
