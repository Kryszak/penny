use application::App;
use runner::run_app;
use std::{io, sync::Arc};
use tokio::sync::Mutex;

mod application;
mod input;
mod runner;

#[tokio::main]
async fn main() -> io::Result<()> {
    let app = Arc::new(Mutex::new(App::new()));

    run_app(&app).await?;

    Ok(())
}
