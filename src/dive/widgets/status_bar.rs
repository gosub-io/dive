use ratatui::prelude::*;
use ratatui::widgets::{Clear, Paragraph};
use crate::dive::app::App;
use crate::dive::ui::get_layout_chunks;
use crate::dive::widget_manager::Drawable;

pub struct StatusBar {
    pub status: String,
}

impl StatusBar {
    pub fn new() -> Self {
        Self {
            status: "Press F1 for help".into(),
        }
    }

    pub fn status(&mut self, status: &str) {
        self.status = status.to_string();
    }
}

impl Drawable for StatusBar {
    fn render(&mut self, _app: &mut App, f: &mut Frame) {
        let chunks = get_layout_chunks(f);

        let status_bar = Paragraph::new(Line::from(vec![
            Span::styled(self.status.clone(), Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" | "),
            Span::raw("Line 1, Column 1"),
        ])).style(Style::default().bg(Color::Blue).bold());

        f.render_widget(Clear, chunks[2]);
        f.render_widget(status_bar, chunks[2]);
    }
}