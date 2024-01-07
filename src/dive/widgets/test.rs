use crate::dive::command_queue::CommandQueue;
use crate::dive::ui::centered_rect;
use crate::dive::widget_manager::Drawable;
use crossterm::event::KeyEvent;
use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph};
use ratatui::Frame;

pub struct TestWidget {
    title: String,
}

impl TestWidget {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.into(),
        }
    }
}

impl Drawable for TestWidget {
    fn render(&mut self, f: &mut Frame) {
        let block = Block::new()
            .title("Test")
            .borders(Borders::ALL)
            .style(
                Style::default()
                    .fg(Color::Yellow)
                    .bold()
                    .bg(Color::LightBlue),
            )
            .border_type(BorderType::Rounded);

        let paragraph = Paragraph::new(self.title.clone())
            .white()
            .on_red()
            .block(block);

        let area = centered_rect(60, 25, f.size());
        f.render_widget(Clear, area);
        f.render_widget(paragraph, area);

        // app.status_bar.status(format!("Opened test screen with {}", self.title).as_str());
    }

    fn event_handler(
        &mut self,
        _queue: &mut CommandQueue,
        _key: KeyEvent,
    ) -> anyhow::Result<Option<KeyEvent>> {
        Ok(None)
    }
}

// impl TestWidget {
//     fn on_show(&mut self, app: &mut App) {
//         app.status_bar.status("Opened test screen");
//     }
//
//     fn on_hide(&mut self, app: &mut App) {
//         app.status_bar.status("Closed test screen");
//     }
// }
