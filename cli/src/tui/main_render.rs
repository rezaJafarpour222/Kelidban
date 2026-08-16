use ratatui::{
    Frame,
    layout::{Constraint, Layout},
};

use crate::tui::{
    context_store::{Context, Screen},
    entries_view::{render_content, render_search, render_status_bar},
};

pub fn render(frame: &mut Frame, context: &mut Context) {
    if context.screen == Screen::Entries {
        let area = frame.area();
        let layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                Constraint::Min(1),
                Constraint::Length(3),
                Constraint::Length(1),
            ])
            .split(area);
        render_content(frame, context, layout[0]);
        render_search(frame, context, layout[1]);
        render_status_bar(frame, context, layout[2]);
    }
}
