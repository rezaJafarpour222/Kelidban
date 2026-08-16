use std::io;

use crate::tui::{context::Action, entry::EntryAction};
use crossterm::event::{self, Event, KeyCode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Entries,
    AddEntry,
}

pub fn input_router(screen: &Screen) -> io::Result<Action> {
    loop {
        if let Event::Key(key) = event::read()? {
            let action = match screen {
                Screen::Entries => read_entites(key.code),
                Screen::AddEntry => read_entry(key.code),
            };
            if let Some(action) = action {
                return Ok(action);
            }
        }
    }
}

pub fn read_entites(key: KeyCode) -> Option<Action> {
    match key {
        KeyCode::Char('q') => Some(Action::Quit),
        KeyCode::Esc => Some(Action::Esc),

        KeyCode::Up => Some(Action::Entries(super::entries::EntriesAction::MoveUp)),
        KeyCode::Down => Some(Action::Entries(super::entries::EntriesAction::MoveDown)),
        KeyCode::Char('a') => Some(Action::EntryScreen),

        KeyCode::Char('y') => Some(Action::Entries(super::entries::EntriesAction::CopyPassword)),

        KeyCode::Char('/') => Some(Action::Search),
        KeyCode::Backspace => Some(Action::DelSearchQuery),

        KeyCode::Char(c) => Some(Action::SearchQuery(c)),
        _ => None,
    }
}
pub fn read_entry(key: KeyCode) -> Option<Action> {
    match key {
        KeyCode::Enter => Some(Action::Entry(EntryAction::GeneratePassword)),
        KeyCode::Char('q') => Some(Action::Quit),
        KeyCode::Esc => Some(Action::Esc),
        KeyCode::Backspace => Some(Action::Entry(EntryAction::Backspace)),
        KeyCode::Up => Some(Action::Entry(EntryAction::Up)),
        KeyCode::Down => Some(Action::Entry(EntryAction::Down)),
        KeyCode::Char(c) => Some(Action::Entry(EntryAction::Input(c))),

        _ => None,
    }
}
