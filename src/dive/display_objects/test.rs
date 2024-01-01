use crossterm::event::KeyEvent;
use ratatui::Frame;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, BorderType, Paragraph};
use crate::dive::app::AppRef;
use crate::dive::obj_manager::Displayable;
use crate::dive::ui::centered_rect;

pub struct TestDisplayObject;

impl TestDisplayObject {
    pub fn new() -> Self {
        Self
    }
}

impl Displayable for TestDisplayObject {
    fn render(&mut self, _app: AppRef, f: &mut Frame) {
        let block = Block::new()
            .title("Test")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Yellow).bold().bg(Color::LightBlue))
            .border_type(BorderType::Rounded)
        ;

        let paragraph = Paragraph::new("Hello Ratatui! (press 'q' to quit)")
            .white()
            .on_red()
            .block(block)
        ;

        let area = centered_rect(60, 25, f.size());
        f.render_widget(paragraph, area);
    }

    fn event_handler(&mut self, _app: AppRef, _key: KeyEvent) -> anyhow::Result<Option<KeyEvent>> {
        Ok(None)
    }

    fn on_show(&mut self, app: AppRef) {
        app.borrow_mut().set_status("Opened test screen");
    }

    fn on_hide(&mut self, app: AppRef) {
        app.borrow_mut().set_status("Closed test screen");
    }
}