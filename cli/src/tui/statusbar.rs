use ratatui::style::{Style, Stylize};
use ratatui::widgets::Borders;
use ratatui::{
    Frame,
    widgets::{Block, Paragraph},
};

use crate::tui::context::Context;
pub fn render_status_bar(frame: &mut Frame, area: ratatui::layout::Rect, guide: String) {
    let guide = Paragraph::new(guide).block(Block::default().red());
    frame.render_widget(guide, area);
}

pub fn render_search(frame: &mut Frame, context: &Context, area: ratatui::layout::Rect) {
    let search = Paragraph::new(context.search.to_string()).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().magenta())
            .title("Search"),
    );
    frame.render_widget(search, area);
}
