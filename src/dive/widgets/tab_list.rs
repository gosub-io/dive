use crossterm::event::KeyEvent;
use ratatui::Frame;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, BorderType, Clear, Paragraph};
use crate::dive::command_queue::CommandQueue;
use crate::dive::ui::centered_rect;
use crate::dive::widget_manager::Drawable;
use crate::dive::widgets::tab_manager::TabManager;

pub struct TabListWidget {
    pub tab_manager: TabManager,
}

impl TabListWidget {
    pub fn new(tab_manager: TabManager) -> Self {
        Self {
            tab_manager,
        }
    }
}

impl Drawable for TabListWidget {
    fn render(&mut self, f: &mut Frame) {
        let block = Block::new()
            .title("Open Tabs")
            .borders(Borders::ALL)
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

    fn event_handler(&mut self, _queue: &mut CommandQueue, _key: KeyEvent) -> anyhow::Result<Option<KeyEvent>> {
        Ok(None)
    }
}