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
                KeyCode::Char('y') => Action::CopyPassword,
                KeyCode::Char('/') => Action::Search,
                KeyCode::Char('a') => Action::EntryRegistery,
                KeyCode::Backspace => Action::DeleteQuery,
                KeyCode::Esc => Action::Esc,
                KeyCode::Char(c) => Action::AddQuery(c),
                _ => continue,
            };

            return Ok(action);
        }
    }
}
