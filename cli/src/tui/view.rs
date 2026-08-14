use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use super::state::State;

pub fn render(frame: &mut Frame, state: &State) {
    let area = frame.area();
    let layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(area);
    render_search(frame, state, layout[0]);
    render_content(frame, state, layout[1]);
    render_status_bar(frame, state, layout[2]);
}

fn render_search(frame: &mut Frame, state: &State, area: ratatui::layout::Rect) {
    let search = Paragraph::new(format!("/{}", state.search))
        .block(Block::default().borders(Borders::ALL).title("Search"));
    frame.render_widget(search, area);
}
fn render_content(frame: &mut Frame, state: &State, area: ratatui::layout::Rect) {
    let layout = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
        .split(area);
    render_entries(frame, state, layout[0]);
    render_details(frame, state, layout[1]);
}

fn render_entries(frame: &mut Frame, state: &State, area: ratatui::layout::Rect) {
    let entries = state.app.entries();
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
        .block(Block::default().borders(Borders::ALL).title("Entries"))
        .highlight_style(Style::default())
        .highlight_symbol("> ");
    let mut state_list = ListState::default();
    state_list.select(if entries.is_empty() {
        None
    } else {
        Some(state.selected)
    });
    frame.render_stateful_widget(list, area, &mut state_list);
}
fn render_details(frame: &mut Frame, state: &State, area: ratatui::layout::Rect) {
    let entry = match state.app.entries().get(state.selected) {
        Some(entry) => entry,
        None => {
            let paragraph = Paragraph::new("No entries")
                .block(Block::default().borders(Borders::ALL).title(" Details "));

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

    let paragraph =
        Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title(" Details "));

    frame.render_widget(paragraph, area);
}

fn field_to_string(value: Option<&[u8]>) -> String {
    value
        .and_then(|value| std::str::from_utf8(value).ok())
        .unwrap_or("")
        .to_string()
}
fn render_status_bar(frame: &mut Frame, state: &State, area: ratatui::layout::Rect) {
    let mode = match state.mode {
        super::state::Mode::Normal => "NORMAL",
        super::state::Mode::Search => "SEARCH",
    };

    let text = format!(" {}   j/k Navigate   / Search   q Quit", mode);

    frame.render_widget(Paragraph::new(text), area);
}
