use std::{
    process::{Command, Stdio},
    time::Duration,
};

use crate::{
    app::{self, App},
    tui::notification::Notification,
};

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Quit,
    MoveUp,
    MoveDown,
    Search,
    AddQuery(char),
    DeleteQuery,
    CopyPassword,
    EntryRegistery,
    Esc,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Search,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Main,
    EntryRegistery,
}
pub struct State {
    pub app: app::App,
    pub mode: Mode,
    pub search: String,
    pub selected: usize,
    pub should_quit: bool,
    pub notification: Option<Notification>,
    pub current_screen: Screen,
}
impl State {
    pub fn new(app: App) -> Self {
        Self {
            should_quit: false,
            app,
            mode: Mode::Normal,
            search: String::from("/"),
            selected: 0,
            current_screen: Screen::Main,
            notification: None,
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
            Action::EntryRegistery => {
                if self.current_screen == Screen::Main {
                    self.current_screen = Screen::EntryRegistery
                } else {
                    if self.mode == Mode::Normal {
                        self.mode = Mode::Search
                    }
                }
            }
            Action::Quit => {
                self.should_quit = true;
            }

            Action::MoveUp => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
            }

            Action::MoveDown => {
                if self.selected + 1 < self.app.entries().len() {
                    self.selected += 1;
                }
            }
            Action::CopyPassword => {
                if self.mode == Mode::Normal {
                    self.copy_password_to_clipboard();
                }
            }
            Action::Search => {
                self.mode = Mode::Search;
                self.search.clear();
            }

            Action::AddQuery(c) => {
                if self.mode == Mode::Search {
                    self.search.push(c);
                }
            }

            Action::DeleteQuery => {
                if self.mode == Mode::Search {
                    self.search.pop();
                }
            }
            Action::Esc => {
                self.mode = Mode::Normal;
                self.search.clear();
                self.search = String::from("/");
                if self.current_screen == Screen::EntryRegistery {
                    self.current_screen = Screen::Main;
                }
            }
            Action::None => {}
        }
    }
    fn copy_password_to_clipboard(&mut self) {
        let Some(entry) = self.app.entries().get(self.selected) else {
            return;
        };
        let Some(password) = entry.password() else {
            return;
        };

        let mut child = match Command::new("wl-copy").stdin(Stdio::piped()).spawn() {
            Ok(child) => child,
            Err(_) => return,
        };

        if let Some(stdin) = child.stdin.as_mut() {
            use std::io::Write;
            let _ = stdin.write_all(password);
        }
        self.notify("Password copied into clipboard");
    }
}
