use ratatui::{
    Frame,
    layout::{Constraint, Layout},
};

use crate::tui::{
    context::Context,
    entries::render_content,
    entry::render_form,
    router::Screen,
    statusbar::{render_search, render_status_bar},
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
        let guide = String::from("j/k:navigation | a:Add");
        render_status_bar(frame, layout[2], guide);
    }
    if context.screen == Screen::AddEntry {
        render_form(frame, frame.area(), context);
    }
}
