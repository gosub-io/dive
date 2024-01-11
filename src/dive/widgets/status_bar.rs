use crate::dive::command_queue::CommandQueue;
use crate::dive::ui::get_layout_chunks;
use crate::dive::widget_manager::Drawable;
use crossterm::event::KeyEvent;
use ratatui::prelude::*;
use ratatui::widgets::{Clear, Paragraph};

pub struct TabInfo {
    pub name: String,
    pub url: String,
    pub secure: bool,
}

pub struct StatusBar {
    pub status: String,
    pub tab_info: Option<TabInfo>,
}

impl StatusBar {
    pub fn new() -> Self {
        Self {
            status: "Press F1 for help".into(),
            tab_info: None,
        }
    }

    pub fn status(&mut self, status: &str) {
        self.status = status.to_string();
    }

    pub fn tab_info(&mut self, tab_info: Option<TabInfo>) {
        self.tab_info = tab_info;
    }
}

impl Drawable for StatusBar {
    fn on_show(&mut self) {}
    fn on_hide(&mut self) {}

    fn render(&mut self, f: &mut Frame) {
        let chunks = get_layout_chunks(f);

        let status_bar = Paragraph::new(Line::from(vec![
            Span::styled(
                self.status.clone(),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(" | "),
            if let Some(tab_info) = &self.tab_info {
                Span::styled(
                    format!(
                        "{} {}",
                        if tab_info.secure { "🔒" } else { "  " },
                        tab_info.url
                    ),
                    Style::default().add_modifier(Modifier::BOLD),
                )
            } else {
                Span::styled(
                    "No tabs open",
                    Style::default().add_modifier(Modifier::BOLD),
                )
            },
        ]))
        .style(Style::default().bg(Color::Blue).bold());

        f.render_widget(Clear, chunks[2]);
        f.render_widget(status_bar, chunks[2]);
    }

    fn event_handler(
        &mut self,
        _queue: &mut CommandQueue,
        _key: KeyEvent,
    ) -> anyhow::Result<Option<KeyEvent>> {
        Ok(None)
    }
}
