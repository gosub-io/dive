use std::cell::RefCell;
use std::rc::Rc;
use crossterm::event;
use crossterm::event::KeyCode::Char;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::event::Event::Key;
use ratatui::Frame;
use crate::dive;
use crate::dive::display_objects::menu::MenuBar;
use crate::dive::display_objects::status::StatusBar;
use crate::dive::obj_manager::{DisplayObject, DisplayObjectManager};
use crate::dive::tab_manager::TabManager;

pub type AppRef = Rc<RefCell<App>>;

pub struct App {
    /// True when the application should exit
    pub should_quit: bool,
    /// Manager for display objects
    pub obj_manager: Rc<RefCell<dive::obj_manager::DisplayObjectManager>>,
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
            should_quit: false,

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

    pub(crate) fn set_status(&mut self, status: &str) {
        self.status_bar.borrow_mut().set_status(status);
    }

    pub(crate) fn handle_events(&self, app: AppRef) -> anyhow::Result<()> {
        if ! event::poll(std::time::Duration::from_millis(250))? {
            return Ok(());
        }

        if let Key(key) = event::read()? {
            if key.kind != event::KeyEventKind::Press {
                return Ok(())
            }

            let res = app.borrow().obj_manager.borrow_mut().active().event_handler(app.clone(), key)?;
            if res.is_none() {
                // The display object did not handle the key, so we should handle it
                self.process_key(app.clone(), key)?;
            }
        }

        Ok(())
    }

    // Renders the screen, and all display objects
    pub(crate) fn render(&self, app: AppRef, f: &mut Frame) {
        let mut objs = Vec::new();

        // Fetch all visible objects
        let binding = &app.borrow().obj_manager;
        let binding = binding.borrow();
        for display_object in binding.objects.iter() {
            if display_object.visible {
                objs.push(display_object);
            }
        }

        // Render all visible display objects
        for display_object in objs.iter() {
            display_object.render(app.clone(), f);
        }
    }

    /// Main key handling
    fn process_key(&self, app: AppRef, key: KeyEvent) -> anyhow::Result<()> {
        match key.code {
            Char(c) if key.modifiers.contains(KeyModifiers::ALT) && c.is_digit(10) => {
                if let Some(digit) = c.to_digit(10) {
                    {
                        let app = app.borrow();
                        let mut tab_manager = app.tab_manager.borrow_mut();
                        tab_manager.switch(digit as usize);
                    }
                    app.borrow().status_bar.borrow_mut().set_status(format!("Switched to tab {}", digit).as_str());
                }
            },
            Char('t') | KeyCode::F(1) => {
                // {
                //     let bm = app.borrow();
                //     let obj = bm.find_display_object_mut("help").unwrap();
                //     obj.show = !obj.show;
                // }

                app.borrow().obj_manager.borrow_mut().visible("help", true);
                app.borrow().obj_manager.borrow_mut().activate("help");
            }
            KeyCode::F(2) => {
                app.borrow().obj_manager.borrow_mut().toggle_visible("test");
                app.borrow().obj_manager.borrow_mut().activate("test");
            }
            // KeyCode::F(9) => self.menu_active = !self.menu_active,
            KeyCode::Tab => {
                let idx;
                {
                    let bm = app.borrow_mut();
                    let mut tab_manager = bm.tab_manager.borrow_mut();
                    idx = tab_manager.next();
                }
                app.borrow_mut().set_status(format!("Switched to tab {}", idx).as_str());
            },

            // Char('i') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            //     // change the name of the current tab
            //     self.popup = true;
            // }
            Char('w') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                let tab_count = self.tab_manager.borrow().tab_count();

                if tab_count > 1 {
                    let idx;
                    {
                        let mut tab_manager = self.tab_manager.borrow_mut();
                        idx = tab_manager.current;
                        tab_manager.close(idx);
                    }
                    app.borrow_mut().set_status(format!("Closed tab {}", idx).as_str());
                } else {
                    app.borrow_mut().set_status("Can't close last tab");
                }
            },
            Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                let idx;
                {
                    let mut tab_manager = self.tab_manager.borrow_mut();
                    idx = tab_manager.add_tab("New Tab", "gosub://blank");
                    tab_manager.switch(idx);
                }
                app.borrow_mut().set_status(format!("Opened new tab {}", idx).as_str());
            },

            Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                app.borrow_mut().should_quit = true;
            }
            _ => {},
        }
        Ok(())
    }
}