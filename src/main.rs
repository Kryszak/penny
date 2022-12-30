use application::App;
use runner::run_app;
use std::io;

mod application;
mod input;
mod runner;

fn main() -> io::Result<()> {
    let mut app = App::new();

    run_app(&mut app)
}
