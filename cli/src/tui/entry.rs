use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Paragraph},
};

use crate::{
    app::App,
    tui::{context::Context, statusbar::render_status_bar},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryType {
    Normal,
    Totp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveField {
    Username,
    Password,
    GeneratePassword,
    Url,
    Notes,
}

pub enum EntryAction {
    Up,
    Down,
    Input(char),
    Backspace,
    GeneratePassword,
}

pub struct EntryForm {
    pub username: String,
    pub password: String,
    pub url: String,
    pub notes: String,
}

impl EntryForm {
    pub fn new() -> Self {
        Self {
            username: String::new(),
            password: String::new(),
            url: String::new(),
            notes: String::new(),
        }
    }
}
pub struct EntryStore {
    pub form: EntryForm,
    pub active: usize,
}

impl EntryStore {
    pub fn new() -> Self {
        Self {
            form: EntryForm::new(),
            active: 0,
        }
    }

    pub fn dispatch(&mut self, action: EntryAction, _app: &App) {
        match action {
            EntryAction::Up => {
                if self.active > 0 {
                    self.active -= 1;
                }
            }
            EntryAction::Down => {
                if self.active < 6 {
                    self.active += 1;
                }
            }
            EntryAction::Input(c) => match self.active {
                0 => self.form.username.push(c),
                1 => self.form.password.push(c),
                2 => {}
                3 => self.form.url.push(c),
                4 => self.form.notes.push(c),
                _ => {}
            },
            EntryAction::Backspace => {
                match self.active {
                    0 => {
                        self.form.username.pop();
                    }
                    1 => {
                        self.form.password.pop();
                    }
                    2 => {
                        // Nothing to delete from Generate Password.
                    }
                    3 => {
                        self.form.url.pop();
                    }
                    4 => {
                        self.form.notes.pop();
                    }
                    _ => {}
                }
            }

            EntryAction::GeneratePassword => {
                if self.active == 2 {
                    self.form.password = generate_password();
                }
                if self.active == 5 {
                    self.form.password = generate_password();
                }

                if self.active == 6 {
                    self.form.password = generate_password();
                }
            }
        }
    }
}

fn generate_password() -> String {
    "GeneratedPassword123!".to_string()
}

// -------------------------
// View
// -------------------------

pub fn render_form(frame: &mut Frame, area: Rect, context: &mut Context) {
    let layout = Layout::vertical([
        Constraint::Min(1),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .split(area);

    form(frame, layout[0], context);

    let guide = String::from("↑/↓:Navigate | Enter:Edit/Generate | Esc:Back");

    render_status_bar(frame, layout[2], guide);
}

fn form(frame: &mut Frame, area: Rect, context: &mut Context) {
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Min(5),
        Constraint::Length(3),
    ])
    .split(area);

    let form = &context.entry_store.form;
    let active = context.entry_store.active;

    let username_block = field_block("Username", active == 0);

    let username = Paragraph::new(form.username.as_str()).block(username_block);

    frame.render_widget(username, chunks[0]);

    let password_chunks =
        Layout::horizontal([Constraint::Min(1), Constraint::Length(22)]).split(chunks[1]);

    let password_block = field_block("Password", active == 1);

    let password = Paragraph::new(form.password.as_str()).block(password_block);

    frame.render_widget(password, password_chunks[0]);

    let generate_block = if active == 2 {
        Block::bordered()
            .title("Generate")
            .border_style(Style::default().fg(Color::Green))
    } else {
        Block::bordered().title("Generate")
    };

    let generate = Paragraph::new("Generate Password").block(generate_block);

    frame.render_widget(generate, password_chunks[1]);

    let url_block = field_block("URL", active == 3);

    let url = Paragraph::new(form.url.as_str()).block(url_block);

    frame.render_widget(url, chunks[2]);

    let notes_block = field_block("Notes", active == 4);

    let notes = Paragraph::new(form.notes.as_str()).block(notes_block);

    frame.render_widget(notes, chunks[3]);

    confirmation(frame, chunks[4], context);
}

fn field_block(title: &str, active: bool) -> Block<'_> {
    if active {
        Block::bordered()
            .title(title)
            .border_style(Style::default().fg(Color::Magenta))
    } else {
        Block::bordered().title(title)
    }
}
fn confirmation(frame: &mut Frame, area: Rect, context: &mut Context) {
    let chunks =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(area);
    let active = context.entry_store.active;

    let save_block = if active == 5 {
        Block::bordered()
            .title("Save")
            .border_style(Style::default().fg(Color::Green))
    } else {
        Block::bordered().title("Save")
    };
    let save = Paragraph::new("Save").block(save_block);

    let cancel_block = if active == 6 {
        Block::bordered()
            .title("Cancel")
            .border_style(Style::default().fg(Color::Red))
    } else {
        Block::bordered().title("Cancel")
    };

    let cancel = Paragraph::new("Cancel").block(cancel_block);
    frame.render_widget(save, chunks[0]);
    frame.render_widget(cancel, chunks[1]);
}
