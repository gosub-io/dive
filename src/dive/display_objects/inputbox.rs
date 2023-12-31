use crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::prelude::{Color, Style, Stylize};
use ratatui::widgets::{Block, Borders};
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;
use crate::dive::app::AppRef;
use crate::dive::display_object::Displayable;
use crate::dive::ui::centered_rect;

pub struct InputBox {
    pub input: Input,
    pub title: String,
    pub on_show_func: Option<Box<dyn Fn(AppRef)>>,
    pub on_hide_func: Option<Box<dyn Fn(AppRef)>>,
}

impl InputBox {
    #[allow(dead_code)]
    pub fn new(
        title: String,
        default_input: Option<String>,
        on_show_func: Option<Box<dyn Fn(AppRef)>>,
        on_hide_func: Option<Box<dyn Fn(AppRef)>>,
    ) -> Self {
        Self {
            input: default_input.unwrap_or("".into()).into(),
            title,
            on_show_func,
            on_hide_func,
        }
    }
}

impl Displayable for InputBox {
    fn render(&mut self, _app: AppRef, f: &mut Frame) {
        let popup_block = Block::default()
            .title(self.title.as_str())
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Yellow).bold().bg(Color::LightBlue));

        let area = centered_rect(60, 25, f.size());
        f.render_widget(popup_block, area);
    }

    fn event_handler(&mut self, _app: AppRef, key: KeyEvent) -> anyhow::Result<Option<KeyEvent>> {
        match key.code {
            KeyCode::Esc => {
                // app.popup = false;
                // app.status = "Cancelled tab rename".into();
            }
            KeyCode::Enter => {
                // app.popup = false;
                // app.status = "Successfully renamed tab".into();
            }
            _ => {
                self.input.handle_event(&Event::Key(key));
            }
        }

        Ok(Some(key))
    }

    fn on_show(&mut self, app: AppRef) {
        self.input = "".into();

        if self.on_show_func.is_some() {
            self.on_show_func.as_ref().unwrap()(app);
        }
    }

    fn on_hide(&mut self, app: AppRef) {
        self.input = "".into();

        if self.on_show_func.is_some() {
            self.on_show_func.as_ref().unwrap()(app);
        }
    }
}