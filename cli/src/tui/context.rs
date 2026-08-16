use crate::{
    app::App,
    tui::{
        entries::{EntriesAction, EntriesStore},
        entry::{EntryAction, EntryStore},
        router::Screen,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Search,
}

pub enum Action {
    Entries(EntriesAction),
    Entry(EntryAction),
    EntryScreen,
    Search,
    SearchQuery(char),
    DelSearchQuery,
    Quit,
    None,
    Esc,
}

pub struct Context {
    pub app: App,
    pub entries_store: EntriesStore,
    pub entry_store: EntryStore,
    pub search: String,
    pub should_quit: bool,
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
            mode: Mode::Normal,
            screen: Screen::Entries,
            action: Action::None,
            entries_store: EntriesStore::new(),
            entry_store: EntryStore::new(),
        }
    }

    pub fn dispatch(&mut self, action: Action) {
        match action {
            Action::Entries(action) => {
                self.entries_store
                    .dispatch(action, &mut self.app, self.mode);
            }
            Action::Entry(action) => self.entry_store.dispatch(action, &mut self.app),

            Action::EntryScreen => self.screen = Screen::AddEntry,

            Action::Quit => {
                self.should_quit = true;
            }
            Action::Esc => {
                if self.screen == Screen::Entries {
                    self.mode = Mode::Normal;
                    self.search = String::from("/");
                }
                if self.screen == Screen::AddEntry {
                    self.screen = Screen::Entries;
                }
            }
            Action::Search => {
                self.mode = Mode::Search;
                self.search.clear();
            }
            Action::SearchQuery(c) => {
                if self.mode == Mode::Search {
                    self.search.push(c);
                }
            }

            Action::DelSearchQuery => {
                if self.mode == Mode::Search {
                    self.search.pop();
                }
            }

            Action::None => {}
        }
    }
}
