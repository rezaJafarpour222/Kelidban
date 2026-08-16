use std::process::{Command, Stdio};

use crate::app::App;
use crate::tui::context::Mode;

#[derive(Debug, Clone, Copy)]
pub enum EntriesAction {
    MoveUp,
    MoveDown,
    CopyPassword,
}

pub struct EntriesStore {
    pub selected: usize,
}
impl EntriesStore {
    pub fn new() -> Self {
        Self { selected: 0 }
    }
    pub fn dispatch(&mut self, action: EntriesAction, app: &App, mode: Mode) {
        match action {
            EntriesAction::MoveUp => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
            }

            EntriesAction::MoveDown => {
                if self.selected + 1 < app.entries().len() {
                    self.selected += 1;
                }
            }
            EntriesAction::CopyPassword => {
                if mode == Mode::Normal {
                    self.copy_password_to_clipboard(app);
                }
            }
        }
    }
    fn copy_password_to_clipboard(&mut self, app: &App) {
        let Some(entry) = app.entries().get(self.selected) else {
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
