use std::io;

use app::App;
use runner::run_app;

mod app;
mod input;
mod runner;

fn main() -> io::Result<()> {
    let mut app = App::new();

    run_app(&mut app)
}
