pub mod app;
pub mod tui;

use std::{io, path::Path};

use ratatui::DefaultTerminal;

use crate::{
    app::App,
    tui::{context_store::Context, main_render},
};

fn main() -> io::Result<()> {
    ratatui::run(run)
}

fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let password = "SecretPassword".as_bytes();
    let path = Path::new("test.kelid");
    let mut app = App::load(path, password)?;
    let mut state = Context::new(app);

    while !state.should_quit {
        terminal.draw(|frame| main_render::render(frame, &mut state))?;

        let action = tui::event::read_action()?;

        state.update(action);
    }

    Ok(())
}
