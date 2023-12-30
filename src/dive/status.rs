use crossterm::event::KeyEvent;
use ratatui::prelude::*;
use ratatui::widgets::Paragraph;
use crate::AppRef;
use crate::dive::display_object::Displayable;

// pub fn status_render(app: &mut App) -> Paragraph<'static> {
//     Paragraph::new(Line::from(vec![
//         Span::styled(app.status.clone(), Style::default().add_modifier(Modifier::BOLD)),
//         Span::raw(" | "),
//         Span::raw("Line 1, Column 1"),
//     ])).style(Style::default().bg(Color::Blue).bold())
// }

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
    fn render(&mut self, app: AppRef, f: &mut Frame) {
        let chunks = app.borrow().get_layout_chunks(f);

        let status_bar = Paragraph::new(Line::from(vec![
            Span::styled(self.status.clone(), Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" | "),
            Span::raw("Line 1, Column 1"),
        ])).style(Style::default().bg(Color::Blue).bold());

        f.render_widget(status_bar, chunks[2]);
    }

    fn event_handler(&mut self, _app: AppRef, _key: KeyEvent) -> anyhow::Result<()> {
        Ok(())
    }

    fn on_show(&mut self, _app: AppRef) { }

    fn on_hide(&mut self, _app: AppRef) { }
}