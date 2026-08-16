pub mod app;
pub mod tui;

use std::{io, path::Path};

use ratatui::DefaultTerminal;

use crate::{
    app::App,
    tui::{context::Context, render, router::input_router},
};

fn main() -> io::Result<()> {
    ratatui::run(run)
}

fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let password = "SecretPassword".as_bytes();
    let path = Path::new("test.kelid");
    let app = App::load(path, password)?;
    let mut state = Context::new(app);

    while !state.should_quit {
        terminal.draw(|frame| render::render(frame, &mut state))?;

        let action = input_router(&state.screen);

        state.dispatch(action.unwrap());
    }

    Ok(())
}
