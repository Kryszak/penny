use std::io;

use application::App;
use runner::run_app;

mod application;
mod files;
mod input;
mod player;
mod runner;

#[tokio::main]
async fn main() -> io::Result<()> {
    match App::new() {
        Some(mut app) => {
            run_app(&mut app).await?;
        }
        None => {
            println!("Failed to open $HOME directory, terminating.")
        }
    };

    Ok(())
}
