use std::{env, io};

use application::App;
use runner::run_app;

mod application;
mod files;
mod input;
mod player;
mod runner;

fn main() -> io::Result<()> {
    let app = env::var("HOME")
        .map(String::from)
        .ok()
        .and_then(|path| App::new(&path));
    match app {
        Some(mut app) => {
            run_app(&mut app)?;
        }
        None => {
            println!("Failed to open starting directory, terminating.")
        }
    };

    Ok(())
}
