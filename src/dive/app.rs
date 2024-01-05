use crossterm::event;
use crossterm::event::KeyCode::Char;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::event::Event::Key;
use crate::dive::widget_manager::{Widget, WidgetManager};
use crate::dive::widgets::help::Help;
use crate::dive::widgets::menu_bar::MenuBar;
use crate::dive::widgets::status_bar::StatusBar;
use crate::dive::widgets::tab_manager::TabManager;
use crate::dive::widgets::test::TestWidget;

pub enum AppState {
    Normal,
    Help,
}

pub struct App {
    pub should_quit: bool,
    pub state: AppState,

    pub status_bar: StatusBar,
    pub menu_bar: MenuBar,
    pub tab_manager: TabManager,
    pub widget_manager: WidgetManager,
}

impl App {
    pub fn new() -> Self {
        let mut app = Self {
            should_quit: false,
            state: AppState::Normal,

            status_bar: StatusBar::new(),
            menu_bar: MenuBar::new(),
            tab_manager: TabManager::new(),
            widget_manager: WidgetManager::new(),
        };

        let inner = Help::new();
        let w1 = Widget::new("help", 0, false, Box::new(inner));
        app.widget_manager.add(w1);

        let inner = TestWidget::new("dos/4gw");
        let w2 = Widget::new("test1", 0, false, Box::new(inner));
        app.widget_manager.add(w2);

        let inner = TestWidget::new("freebsd");
        let w3 = Widget::new("test2", 0, false, Box::new(inner));
        app.widget_manager.add(w3);

        app
    }

    pub(crate) fn handle_events(&mut self) -> anyhow::Result<()> {
        if ! event::poll(std::time::Duration::from_millis(250))? {
            return Ok(());
        }

        if let Key(key) = event::read()? {
            if key.kind != event::KeyEventKind::Press {
                return Ok(())
            }

            match self.state {
                AppState::Normal => {
                    self.process_key(key)?;
                },
                // AppState::HelpPopup => {
                //     self.process_key(key)?;
                // },
                AppState::Help => {
                    self.widget_manager.find("help").unwrap().inner.event_handler(self, key)?;
                },
            }
        }

        Ok(())
    }

    /// Main key handling
    fn process_key(&mut self, key: KeyEvent) -> anyhow::Result<()> {
        match key.code {
            Char(c) if key.modifiers.contains(KeyModifiers::ALT) && c.is_digit(10) => {
                if let Some(digit) = c.to_digit(10) {
                    self.tab_manager.switch(digit as usize);
                    self.status_bar.status(format!("Switched to tab {}", digit).as_str());
                }
            },
            Char('t') | KeyCode::F(1) => {
                self.state = AppState::Menu;
                self.widget_manager.show("help");
                self.widget_manager.focus("help");
            }
            KeyCode::F(2) => {
                self.widget_manager.find("test1").unwrap().toggle();
            }
            KeyCode::F(3) => {
                self.widget_manager.find("test2").unwrap().toggle();
            }
            // KeyCode::F(9) => self.menu_active = !self.menu_active,
            KeyCode::Tab if key.modifiers.contains(KeyModifiers::SHIFT) => {
                let idx = self.tab_manager.prev();
                self.status_bar.status(format!("Switched to tab {}", idx).as_str());
            },
            KeyCode::Tab => {
                let idx = self.tab_manager.next();
                self.status_bar.status(format!("Switched to tab {}", idx).as_str());
            },
            // Char('i') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            //     // change the name of the current tab
            //     self.popup = true;
            // }
            Char('w') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                if self.tab_manager.len() == 1 {
                    self.status_bar.status("Can't close last tab");
                    return Ok(());
                }

                let idx = self.tab_manager.current;
                self.tab_manager.close(idx);
                self.status_bar.status(format!("Closed tab {}", idx).as_str());
            },
            Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                let idx = self.tab_manager.open("New Tab", "gosub://blank");
                self.tab_manager.switch(idx);
                self.status_bar.status(format!("Opened new tab {}", idx).as_str());
            },

            Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.should_quit = true;
            }
            _ => {},
        }
        Ok(())
    }
}