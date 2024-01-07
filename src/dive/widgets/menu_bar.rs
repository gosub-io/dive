use crate::dive::command_queue::CommandQueue;
use crate::dive::ui::get_layout_chunks;
use crate::dive::widget_manager::Drawable;
use crossterm::event::KeyEvent;
use ratatui::prelude::*;
use ratatui::widgets::{Clear, Paragraph};

pub struct MenuBar {
    pub active: bool,
    pub menu_item_active: u8,
}

impl MenuBar {
    pub fn new() -> Self {
        Self {
            active: false,
            menu_item_active: 0,
        }
    }

    #[allow(dead_code)]
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }
}

impl Drawable for MenuBar {
    fn render(&mut self, f: &mut Frame) {
        let menu_items = ["File",
            "Edit",
            "View",
            "History",
            "Bookmarks",
            "Tools",
            "Help"];

        let mut menu_tiles = vec![Span::styled(
            " Gosub Dive ",
            Style::default().fg(Color::White).bold(),
        )];

        for (idx, item) in menu_items.iter().enumerate() {
            menu_tiles.push(Span::raw("|"));

            if self.active && self.menu_item_active == idx as u8 {
                menu_tiles.push(Span::styled(
                    format!(" {} ", item),
                    Style::default()
                        .bg(Color::Green)
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ));
            } else {
                menu_tiles.push(Span::raw(format!(" {} ", item)));
            }
        }

        let chunks = get_layout_chunks(f);
        let menu_bar = Paragraph::new(Line::from(menu_tiles)).style(
            Style::default()
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        );

        f.render_widget(Clear, chunks[0]);
        f.render_widget(menu_bar, chunks[0]);
    }

    fn event_handler(
        &mut self,
        _queue: &mut CommandQueue,
        _key: KeyEvent,
    ) -> anyhow::Result<Option<KeyEvent>> {
        Ok(None)
    }
}
