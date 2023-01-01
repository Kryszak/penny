use application::App;
use runner::run_app;
use std::{io, sync::Arc};
use tokio::sync::Mutex;

mod application;
mod files;
mod input;
mod player;
mod runner;

#[tokio::main]
async fn main() -> io::Result<()> {
    match App::new() {
        Some(app) => {
            run_app(&Arc::new(Mutex::new(app))).await?;
        }
        None => {println!("Failed to open $HOME directory, terminating.")}
    };

    Ok(())
}
