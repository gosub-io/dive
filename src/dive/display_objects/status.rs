use crossterm::event::KeyEvent;
use ratatui::prelude::*;
use ratatui::widgets::Paragraph;
use crate::dive::app::AppRef;
use crate::dive::obj_manager::Displayable;
use crate::dive::ui::get_layout_chunks;

pub struct StatusBar {
    pub status: String,
}

impl StatusBar {
    pub fn new() -> Self {
        Self {
            status: "Press F1 for help".into(),
        }
    }

    pub fn set_status(&mut self, status: &str) {
        self.status = status.to_string();
    }
}

impl Displayable for StatusBar {
    fn render(&mut self, _app: AppRef, f: &mut Frame) {
        let chunks = get_layout_chunks(f);

        let status_bar = Paragraph::new(Line::from(vec![
            Span::styled(self.status.clone(), Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" | "),
            Span::raw("Line 1, Column 1"),
        ])).style(Style::default().bg(Color::Blue).bold());

        f.render_widget(status_bar, chunks[2]);
    }

    fn event_handler(&mut self, _app: AppRef, _key: KeyEvent) -> anyhow::Result<Option<KeyEvent>> {
        // Status bar does not handle keys
        Ok(None)
    }

    fn on_show(&mut self, _app: AppRef) { }

    fn on_hide(&mut self, _app: AppRef) { }
}