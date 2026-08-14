use ratatui::{
    Frame,
    widgets::{Block, Borders, List, ListItem, ListState},
};

use super::state::State;

pub fn render(frame: &mut Frame, state: &State) {
    let items: Vec<ListItem> = state
        .app
        .entries()
        .iter()
        .map(|entry| {
            let title = entry
                .title()
                .and_then(|title| std::str::from_utf8(title).ok())
                .unwrap_or("Untitled");

            ListItem::new(title)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().title("Kelidban").borders(Borders::ALL))
        .highlight_symbol("> ");

    let mut list_state = ListState::default();

    if !state.app.entries().is_empty() {
        list_state.select(Some(state.selected));
    }

    frame.render_stateful_widget(list, frame.area(), &mut list_state);
}
