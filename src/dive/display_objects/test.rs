use crossterm::event::KeyEvent;
use ratatui::Frame;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, BorderType, Clear, Paragraph};
use crate::dive::app::App;
use crate::dive::ui::centered_rect;
use crate::dive::widget_manager::Widget;

pub struct TestComponent {
    title: String,
}

impl TestComponent {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.into(),
        }
    }
}

impl Widget for TestComponent {
    fn render(&mut self, _app: &mut App, f: &mut Frame) {
        let block = Block::new()
            .title("Test")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Yellow).bold().bg(Color::LightBlue))
            .border_type(BorderType::Rounded)
        ;

        let paragraph = Paragraph::new(self.title.clone())
            .white()
            .on_red()
            .block(block)
        ;

        let area = centered_rect(60, 25, f.size());
        f.render_widget(Clear, area);
        f.render_widget(paragraph, area);
    }

    fn event_handler(&mut self, _app: &mut App, _key: KeyEvent) -> anyhow::Result<Option<KeyEvent>> {
        Ok(None)
    }

    fn on_show(&mut self, app: &mut App) {
        app.status_bar.status("Opened test screen");
    }

    fn on_hide(&mut self, app: &mut App) {
        app.status_bar.status("Closed test screen");
    }
}