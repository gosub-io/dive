use crate::dive::command_queue::{Command, CommandQueue};
use crate::dive::ui::centered_rect_fixed;
use crate::dive::widget_manager::Drawable;
use crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Clear, Padding, Paragraph};
use ratatui::Frame;
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;

pub struct InputWidget {
    title: String,
    input: Input,
    max_size: usize,
    command: InputSubmitCommand,
}

#[derive(Clone, Debug, PartialEq)]
pub enum InputSubmitCommand {
    RenameTab { tab_idx: usize },
}

impl InputWidget {
    pub fn new(title: &str, value: &str, max_size: usize, command: InputSubmitCommand) -> Self {
        Self {
            title: title.into(),
            input: Input::new(value.into()),
            max_size,
            command,
        }
    }
}

impl Drawable for InputWidget {
    fn on_show(&mut self) {}
    fn on_hide(&mut self) {}

    fn render(&mut self, f: &mut Frame) {
        let block = Block::new()
            .title(format!(" {} ", self.title.as_str()))
            .borders(Borders::ALL)
            .style(
                Style::default()
                    .fg(Color::Yellow)
                    .bold()
                    .bg(Color::LightBlue),
            )
            .padding(Padding::new(1, 1, 1, 1));

        let input = Paragraph::new(self.input.value().to_string())
            .style(Style::default().bg(Color::Blue).fg(Color::Yellow))
            .block(block);

        let area = centered_rect_fixed(self.max_size as u16, 5, f.size());
        f.render_widget(Clear, area);
        f.render_widget(input, area);

        f.set_cursor(area.x + self.input.value().len() as u16 + 2, area.y + 2);
    }

    fn event_handler(
        &mut self,
        queue: &mut CommandQueue,
        key: KeyEvent,
    ) -> anyhow::Result<Option<KeyEvent>> {
        match key.code {
            KeyCode::Esc => {
                queue.push(Command::DestroyWidget { id: "input".into() });
            }
            KeyCode::Enter => {
                queue.push(Command::InputSubmit {
                    command: self.command.clone(),
                    value: self.input.to_string(),
                });
                queue.push(Command::DestroyWidget { id: "input".into() });
            }
            _ => {
                self.input.handle_event(&Event::Key(key));
            }
        }

        Ok(Some(key))
    }
}
