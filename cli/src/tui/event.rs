use std::io;

use crossterm::event::{self, Event, KeyCode};

use crate::tui::context::Action;

pub fn read_action() -> io::Result<Action> {
    loop {
        if let Event::Key(key) = event::read()? {
            let action = match key.code {
                KeyCode::Char('q') => Action::Quit,
                KeyCode::Up | KeyCode::Char('k') => {
                    Action::Entries(super::entries_state::EntriesAction::MoveUp)
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    Action::Entries(super::entries_state::EntriesAction::MoveDown)
                }
                KeyCode::Char('y') => {
                    Action::Entries(super::entries_state::EntriesAction::MoveDown)
                }

                KeyCode::Char('/') => Action::Search,
                KeyCode::Backspace => Action::DelSearchQuery,

                KeyCode::Char(c) => Action::SearchQuery(c),
                KeyCode::Esc => Action::Esc,
                _ => continue,
            };

            return Ok(action);
        }
    }
}
