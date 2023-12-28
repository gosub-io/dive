use ratatui::prelude::*;
use ratatui::widgets::Paragraph;
use crate::dive::app::App;

pub fn status_render(app: &mut App) -> Paragraph<'static> {
    Paragraph::new(Line::from(vec![
        Span::styled(app.status.clone(), Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" | "),
        Span::raw("Line 1, Column 1"),
    ])).style(Style::default().bg(Color::Blue).bold())
}