use crossterm::event::{KeyCode, KeyEvent};
use crossterm::event::KeyCode::Char;
use ratatui::Frame;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, BorderType, Clear, Paragraph};
use crate::dive::command_queue::{Command, CommandQueue};
use crate::dive::ui::centered_rect;
use crate::dive::widget_manager::Drawable;

const SPLASH_TEXT: &str = r#"
  _____ _____ _____ _   _______      ______ _____ _   _ _____
  |  __ \  _  /  ___| | | | ___ \     |  _  \_   _| | | |  ___|
 | |  \/ | | \ `--.| | | | |_/ /     | | | | | | | | | | |__
 | | __| | | |`--. \ | | | ___ \     | | | | | | | | | |  __|
 | |_\ \ \_/ /\__/ / |_| | |_/ /     | |/ / _| |_\ \_/ / |___
  \____/\___/\____/ \___/\____/      |___/  \___/ \___/\____/

Press ESC to continue

The text based browser based on the Gosub engine
Copyright (C) 2024 - The Gosub Community

"#;

pub struct SplashWidget;

impl SplashWidget {
    pub fn new() -> Self {
        Self
    }
}

impl Drawable for SplashWidget {
    fn render(&mut self, f: &mut Frame) {
        let block = Block::new()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White).bold().bg(Color::Black))
            .border_type(BorderType::Rounded)
        ;

        let paragraph = Paragraph::new(SPLASH_TEXT)
            .white()
            .block(block)
            .alignment(Alignment::Center)
        ;

        let area = centered_rect(60, 30, f.size());
        f.render_widget(Clear, area);
        f.render_widget(paragraph, area);
    }

    fn event_handler(&mut self, queue: &mut CommandQueue, key: KeyEvent) -> anyhow::Result<Option<KeyEvent>> {
        match key.code {
            KeyCode::Esc | Char(' ') => {
                queue.push(Command::DestroyWidget{id: "splash".into()});
            }
            _ => {}
        }

        Ok(Some(key))
    }
}