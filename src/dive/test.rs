use crossterm::event::KeyEvent;
use ratatui::Frame;
use ratatui::prelude::*;
use ratatui::widgets::Paragraph;
use crate::AppRef;
use crate::dive::display_object::Displayable;

pub struct TestDisplayObject;

impl TestDisplayObject {
    pub fn new() -> Self {
        Self
    }
}

impl Displayable for TestDisplayObject {
    fn render(&mut self, _app: AppRef, f: &mut Frame) {
        let area = f.size();
        f.render_widget(
            Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                .white()
                .on_blue(),
            area,
        );
    }

    fn event_handler(&mut self, _app: AppRef, _key: KeyEvent) -> anyhow::Result<()> {
        Ok(())
    }

    fn on_show(&mut self, app: AppRef) {
        app.borrow_mut().set_status("Opened test screen");
    }

    fn on_hide(&mut self, app: AppRef) {
        app.borrow_mut().set_status("Closed test screen");
    }
}