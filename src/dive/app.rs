use std::cell::RefCell;
use std::rc::Rc;
use crossterm::event::KeyCode::Char;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::dive;
use crate::dive::display_objects::menu::MenuBar;
use crate::dive::display_objects::status::StatusBar;
use crate::dive::obj_manager::{DisplayObject, DisplayObjectManager};
use crate::dive::tab_manager::TabManager;

pub type AppRef = Rc<RefCell<App>>;

pub struct AppVars {
    /// True when the application should exit
    pub should_quit: bool,
}

pub struct App {
    pub vars: AppVars,
    /// Manager for display objects
    pub obj_manager: Rc<RefCell<DisplayObjectManager>>,
    /// Status bar object
    pub status_bar: Rc<RefCell<StatusBar>>,
    /// Menu bar object
    pub menu_bar: Rc<RefCell<MenuBar>>,
    /// TabLayout object
    pub tab_manager: Rc<RefCell<TabManager>>,
}

impl App {
    pub fn new() -> AppRef {
        let status_bar = Rc::new(RefCell::new(StatusBar::new()));
        let menu_bar = Rc::new(RefCell::new(MenuBar::new()));
        let tab_manager = Rc::new(RefCell::new(TabManager::new()));
        let obj_manager = Rc::new(RefCell::new(DisplayObjectManager::new()));

        let app = App {
            vars: AppVars {
                should_quit: false
            },
            status_bar: status_bar.clone(),
            menu_bar: menu_bar.clone(),
            tab_manager: tab_manager.clone(),
            obj_manager: obj_manager.clone(),
        };

        let app_ref = Rc::new(RefCell::new(app));

        let tab_display_obj= Rc::new(RefCell::new(dive::display_objects::tab_display::TabDisplay::new(tab_manager.clone())));

        // Add display objects
        {
            let binding = app_ref.borrow();
            let mut obj_manager = binding.obj_manager.borrow_mut();
            obj_manager.add(DisplayObject::new("menu", 128, menu_bar.clone(), true));
            obj_manager.add(DisplayObject::new("status", 128, status_bar.clone(), true));
            obj_manager.add(DisplayObject::new("tabs", 0, tab_display_obj.clone(), true));

            let test = Rc::new(RefCell::new(dive::display_objects::test::TestDisplayObject::new()));
            obj_manager.add(DisplayObject::new("test", 64, test.clone(), false));

            let help = Rc::new(RefCell::new(dive::display_objects::help::HelpDisplayObject::new()));
            obj_manager.add(DisplayObject::new("help", 0, help.clone(), false));
        }

        app_ref
    }

    // pub(crate) fn set_status(&mut self, status: &str) {
    //     self.status_bar.borrow_mut().status(status);
    // }

    /// Main key handling
    pub(crate) fn process_key(&mut self, _app: AppRef, key: KeyEvent) -> anyhow::Result<()> {
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
                self.obj_manager.borrow_mut().visible("help", true);
                self.obj_manager.borrow_mut().activate("help");
            }
            KeyCode::F(2) => {
                self.obj_manager.borrow_mut().toggle_visible("test");
                self.obj_manager.borrow_mut().activate("test");
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