pub mod app;
pub mod tui;

use std::{io, path::Path};

use encryption::file::vault::Vault;
use ratatui::DefaultTerminal;

use crate::app::App;

fn main() -> io::Result<()> {
    ratatui::run(run)
}

fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let password = "SecretPassword".as_bytes();
    let path = Path::new("test.kelid");
    let mut app = App::load(path, password)?;
    let mut state = tui::state::State::new(app);

    while !state.should_quit {
        terminal.draw(|frame| tui::view::render(frame, &mut state))?;

        let action = tui::event::read_action()?;

        state.update(action);
    }

    Ok(())
}
