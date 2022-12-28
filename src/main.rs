use std::{env, io};

use app_state::FileViewerList;

mod app;
mod app_state;
mod ui;

fn main() -> io::Result<()> {
    let mut app_state = crate::app_state::AppState {
        help_visible: true,
        logs_visible: false,
        file_list: FileViewerList::with_directory(&env::var("HOME").unwrap()),
    };

    app::run_app(&mut app_state)
}
