use crossterm::event::KeyEvent;
use ratatui::Frame;
use ratatui::prelude::*;
use ratatui::widgets::Paragraph;
use crate::dive::app::App;
use crate::dive::display_object::Displayable;

struct TestDisplayObject;

impl TestDisplayObject {
    pub fn new() -> Self {
        Self
    }
}

impl Displayable for TestDisplayObject {
    fn render(&mut self, _app: &mut App, f: &mut Frame) {
        let area = f.size();
        f.render_widget(
            Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                .white()
                .on_blue(),
            area,
        );
    }

    fn event_handler(&mut self, _app: &mut App, _key: KeyEvent) -> anyhow::Result<()> {
        Ok(())
    }

    fn on_show(&mut self, app: &mut App) {
        app.status = "Opened test screen".into();
    }

    fn on_hide(&mut self, app: &mut App) {
        app.status = "Closed test screen".into();
    }
}