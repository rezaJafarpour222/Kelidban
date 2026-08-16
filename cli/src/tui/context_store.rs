use std::time::Duration;

use crate::{
    app::App,
    tui::{
        entries_state::{EntriesAction, EntriesStore},
        notification::Notification,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Search,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Entries,
    AddEntry,
}
pub enum Action {
    Entries(EntriesAction),
    Quit,
    None,
    Esc,
}

pub struct Context {
    pub app: App,
    pub entry_store: EntriesStore,
    pub search: String,
    pub should_quit: bool,
    pub notification: Option<Notification>,
    pub mode: Mode,
    pub screen: Screen,
    pub action: Action,
}

impl Context {
    pub fn new(app: App) -> Self {
        Self {
            app,
            search: String::from("/"),
            should_quit: false,
            notification: None,
            mode: Mode::Normal,
            screen: Screen::Entries,
            action: Action::None,
            entry_store: EntriesStore::new(),
        }
    }

    pub fn notify(&mut self, message: &str) {
        self.notification = Some(Notification::new(message, Duration::from_millis(500)));
    }
    pub fn notification(&mut self) -> Option<&str> {
        if self.notification.as_ref().is_some_and(|n| n.is_expired()) {
            self.notification = None;
            return None;
        }
        self.notification.as_ref().map(|n| n.get_message())
    }
    pub fn update(&mut self, action: Action) {
        match action {
            Action::Quit => {
                self.should_quit = true;
            }
            Action::Entries(action) => self.entry_store.update(action, &self.app, self.mode),
            Action::Esc => self.mode = Mode::Normal,

            Action::None => {}
        }
    }
}
