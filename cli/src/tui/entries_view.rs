use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use crate::tui::context_store::{Context, Mode};

pub fn render_search(frame: &mut Frame, context: &Context, area: ratatui::layout::Rect) {
    let search = Paragraph::new(context.search.to_string()).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().magenta())
            .title("Search"),
    );
    frame.render_widget(search, area);
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
        Some(context.entry_store.selected)
    });
    frame.render_stateful_widget(list, area, &mut state_list);
}
pub fn render_details(frame: &mut Frame, context: &Context, area: ratatui::layout::Rect) {
    let entry = match context.app.entries().get(context.entry_store.selected) {
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
pub fn render_status_bar(frame: &mut Frame, context: &mut Context, area: ratatui::layout::Rect) {
    let mode = match context.mode {
        Mode::Normal => "NORMAL",
        Mode::Search => "SEARCH",
    };
    let message = context.notification().unwrap_or("");
    let text = format!(
        " {}   j/k Navigate   / Search   q Quit a Add -|  {}",
        mode, message
    );
    let paragraph = Paragraph::new(text).block(Block::default().red());
    frame.render_widget(paragraph, area);
}
// NOTE: Add Entry Form
