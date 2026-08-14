use std::io;

use crossterm::event::{self, Event, KeyCode};

use crate::tui::state::Action;

pub fn read_action() -> io::Result<Action> {
    loop {
        if let Event::Key(key) = event::read()? {
            let action = match key.code {
                KeyCode::Char('q') => Action::Quit,
                KeyCode::Up | KeyCode::Char('k') => Action::MoveUp,
                KeyCode::Down | KeyCode::Char('j') => Action::MoveDown,
                KeyCode::Char('/') => Action::Search,
                _ => continue,
            };

            return Ok(action);
        }
    }
}
