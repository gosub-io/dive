use crossterm::event;
use crossterm::event::KeyCode::Char;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::event::Event::Key;
use ratatui::Frame;
use crate::dive::display_objects::help::HelpComponent;
use crate::dive::display_objects::test::TestComponent;
use crate::dive::widget_manager::{WidgetManager, WidgetObject};
use crate::dive::widgets::menu_bar::MenuBar;
use crate::dive::widgets::status_bar::StatusBar;
use crate::dive::widgets::tab_manager::TabManager;

pub enum AppState {
    Normal,
    // HelpPopup,
    // MenuActive,
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
    pub fn new() -> App {

        let mut app = App {
            should_quit: false,
            state: AppState::Normal,

            status_bar: StatusBar::new(),
            menu_bar: MenuBar::new(),
            tab_manager: TabManager::new(),
            widget_manager: WidgetManager::new(),
        };

        let w1 = WidgetObject::new("help", 0, Box::new(HelpComponent::new()), false);
        app.widget_manager.add(w1);
        let w2 = WidgetObject::new("test1", 0, Box::new(TestComponent::new("dos/4gw")), false);
        app.widget_manager.add(w2);
        let w3 = WidgetObject::new("test2", 0, Box::new(TestComponent::new("foobar!")), false);
        app.widget_manager.add(w3);

        app
    }
    pub(crate) fn render(&mut self, f: &mut Frame) {
        self.tab_manager.render(f);
        self.status_bar.render(f);
        self.menu_bar.render(f);

        self.widget_manager.render(self, f);
    }

    pub(crate) fn handle_events(&mut self) -> anyhow::Result<()> {
        if ! event::poll(std::time::Duration::from_millis(250))? {
            return Ok(());
        }

        if let Key(key) = event::read()? {
            if key.kind != event::KeyEventKind::Press {
                return Ok(())
            }

            self.process_key(key)?;
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
                self.widget_manager.show("help");
                self.widget_manager.focus("help");
            }
            KeyCode::F(2) => {
                self.widget_manager.toggle("test1");
            }
            KeyCode::F(3) => {
                self.widget_manager.toggle("test2");
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