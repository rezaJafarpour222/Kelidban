use std::process::{Command, Stdio};

use crate::app::{self, App};

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Quit,
    MoveUp,
    MoveDown,
    Search,
    CopyPassword,
    Esc,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Search,
}
pub struct State {
    pub app: app::App,
    pub mode: Mode,
    pub search: String,
    pub selected: usize,
    pub should_quit: bool,
}
impl State {
    pub fn new(app: App) -> Self {
        Self {
            should_quit: false,
            app,
            mode: Mode::Normal,
            search: String::new(),
            selected: 0,
        }
    }
    pub fn update(&mut self, action: Action) {
        match action {
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
            }
            Action::Esc => {
                self.mode = Mode::Normal;
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
    }
}
