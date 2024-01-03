use std::cell::RefCell;
use std::rc::Rc;
use crossterm::event;
use crossterm::event::KeyCode::Char;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::event::Event::Key;
use ratatui::Frame;
use crate::dive::display_objects::menu::MenuBar;
use crate::dive::display_objects::status::StatusBar;
use crate::dive::tab_manager::TabManager;

pub enum AppState {
    Normal,
    HelpPopup,
    MenuActive,
}

pub struct App {
    pub should_quit: bool,
    pub state: AppState,

    pub status_bar: StatusBar,
    pub menu_bar: MenuBar,
    pub tab_manager: TabManager,
}

impl App {
    pub fn new() -> App {
        App {
            should_quit: false,
            state: AppState::Normal,

            status_bar: StatusBar::new(),
            menu_bar: MenuBar::new(),
            tab_manager: TabManager::new(),
        }
    }

    pub(crate) fn visible(&mut self, id: &str, visible: bool) {
        match id {
            "help" => self.state = if visible { AppState::HelpPopup } else { AppState::Normal },
            _ => {},
        }
    }

    pub(crate) fn status(&mut self, status: &str) {
        self.status_bar.status(status);
    }

    pub(crate) fn render(&mut self, f: &mut Frame) {
        let mut tab_manager = self.tab_manager.borrow_mut();
        let mut status_bar = self.status_bar.borrow_mut();
        let mut menu_bar = self.menu_bar.borrow_mut();

        tab_manager.render(f.clone());
        status_bar.render(f.clone());
        menu_bar.render(f.clone());
    }

    pub(crate) fn event(&mut self) -> anyhow::Result<()> {
        if ! event::poll(std::time::Duration::from_millis(250))? {
            return Ok(());
        }

        if let Key(key) = event::read()? {
            if key.kind != event::KeyEventKind::Press {
                return Ok(())
            }

            match self.state {
                AppState::Normal => self.process_key(&mut self, key)?,
                AppState::HelpPopup => help.process_key(app, key)?;
                _ => {},
            }
        }

        Ok(())
    }

    /// Main key handling
    fn process_key(&mut self, key: KeyEvent) -> anyhow::Result<()> {
        match key.code {
            Char(c) if key.modifiers.contains(KeyModifiers::ALT) && c.is_digit(10) => {
                if let Some(digit) = c.to_digit(10) {
                    {
                        let mut tab_manager = self.tab_manager.borrow_mut();
                        tab_manager.switch(digit as usize);
                    }
                    self.status_bar.borrow_mut().status(format!("Switched to tab {}", digit).as_str());
                }
            },
            Char('t') | KeyCode::F(1) => {
                self.visible("help", true);
                self.activate("help");
            }
            KeyCode::F(2) => {
                self.toggle_visible("test");
                self.activate("test");
            }
            // KeyCode::F(9) => self.menu_active = !self.menu_active,
            KeyCode::Tab => {
                let idx;
                {
                    let mut tab_manager = self.tab_manager.borrow_mut();
                    idx = tab_manager.next();
                }
                self.status_bar.borrow_mut().status(format!("Switched to tab {}", idx).as_str());
            },
            // Char('i') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            //     // change the name of the current tab
            //     self.popup = true;
            // }
            Char('w') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                let tab_count = self.tab_manager.borrow().tab_count();

                if tab_count == 1 {
                    self.status_bar.borrow_mut().status("Can't close last tab");
                    return Ok(());
                }

                let idx;
                {
                    let mut tab_manager = self.tab_manager.borrow_mut();
                    idx = tab_manager.current;
                    tab_manager.close(idx);
                }
                self.status_bar.borrow_mut().status(format!("Closed tab {}", idx).as_str());
            },
            Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                let idx;
                {
                    let mut tab_manager = self.tab_manager.borrow_mut();
                    idx = tab_manager.add_tab("New Tab", "gosub://blank");
                    tab_manager.switch(idx);
                }
                self.status_bar.borrow_mut().status(format!("Opened new tab {}", idx).as_str());
            },

            Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.vars.should_quit = true;
            }
            _ => {},
        }
        Ok(())
    }
}