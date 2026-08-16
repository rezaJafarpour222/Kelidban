use crate::app::App;
use crate::tui::context::Context;
use crate::tui::context::Mode;
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use std::process::{Command, Stdio};

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
    pub fn dispatch(&mut self, action: EntriesAction, app: &App, mode: Mode) -> Option<String> {
        match action {
            EntriesAction::MoveUp => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
                None
            }

            EntriesAction::MoveDown => {
                if self.selected + 1 < app.entries().len() {
                    self.selected += 1;
                }
                None
            }
            EntriesAction::CopyPassword => {
                if mode == Mode::Normal {
                    self.copy_password_to_clipboard(app);
                }
                Some(String::from("Password copied!"))
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
pub fn render_content(frame: &mut Frame, context: &Context, area: ratatui::layout::Rect) {
    let layout = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
        .split(area);
    render_entries(frame, context, layout[0]);
    render_details(frame, context, layout[1]);
}

pub fn render_entries(frame: &mut Frame, context: &Context, area: ratatui::layout::Rect) {
    let entries = context.app.entries();
    let items: Vec<ListItem> = entries
        .iter()
        .map(|entry| {
            let title = entry
                .title()
                .and_then(|value| std::str::from_utf8(value).ok())
                .unwrap_or("<untitled>");
            ListItem::new(title)
        })
        .collect();
    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Entries")
                .border_style(Style::default().magenta()),
        )
        .highlight_style(Style::default())
        .highlight_symbol("> ")
        .highlight_style(Style::new().red());
    let mut state_list = ListState::default();
    state_list.select(if entries.is_empty() {
        None
    } else {
        Some(context.entries_store.selected)
    });
    frame.render_stateful_widget(list, area, &mut state_list);
}
pub fn render_details(frame: &mut Frame, context: &Context, area: ratatui::layout::Rect) {
    let entry = match context.app.entries().get(context.entries_store.selected) {
        Some(entry) => entry,
        None => {
            let paragraph = Paragraph::new("No entries").block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().magenta())
                    .title(" Details "),
            );

            frame.render_widget(paragraph, area);
            return;
        }
    };

    let title = field_to_string(entry.title());
    let username = field_to_string(entry.username());
    let url = field_to_string(entry.url());
    let notes = field_to_string(entry.notes());

    let lines = vec![
        Line::from(vec![Span::raw("Title:    "), Span::raw(title)]),
        Line::from(vec![Span::raw("Username: "), Span::raw(username)]),
        Line::from(vec![Span::raw("Password: "), Span::raw("********")]),
        Line::from(vec![Span::raw("URL:      "), Span::raw(url)]),
        Line::from(vec![Span::raw("Notes:    "), Span::raw(notes)]),
    ];

    let paragraph = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().magenta())
            .red()
            .title(" Details "),
    );

    frame.render_widget(paragraph, area);
}

fn field_to_string(value: Option<&[u8]>) -> String {
    value
        .and_then(|value| std::str::from_utf8(value).ok())
        .unwrap_or("")
        .to_string()
}
// NOTE: Add Entry Form
