pub mod app;
pub mod tui;

use std::io;

use encryption::file::vault::Vault;
use ratatui::DefaultTerminal;

use crate::app::App;

fn main() -> io::Result<()> {
    ratatui::run(run)
}

fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let mut app = App::new(Vault::new());

    app.add_entry(
        "GitHub",
        "user123",
        "secret",
        "https://github.com",
        "GitHub account",
    );

    app.add_entry(
        "Bank",
        "bankuser",
        "bankpass",
        "https://bank.example",
        "Bank account",
    );

    let mut state = tui::state::State::new(app);

    while !state.should_quit {
        terminal.draw(|frame| tui::view::render(frame, &state))?;

        let action = tui::event::read_action()?;

        state.update(action);
    }

    Ok(())
}
