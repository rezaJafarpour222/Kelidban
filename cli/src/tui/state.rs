use crate::app::{self, App};

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Quit,
    MoveUp,
    MoveDown,
    Search,
    Esc,
    None,
}
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

            Action::Search => {
                self.mode = Mode::Search;
            }
            Action::Esc => {
                self.mode = Mode::Normal;
            }
            Action::None => {}
        }
    }
}
