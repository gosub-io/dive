use std::cell::RefCell;
use std::rc::Rc;
use crossterm::event;
use crossterm::event::KeyCode::Char;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::event::Event::Key;
use ratatui::Frame;
use crate::dive;
use crate::dive::display_object::DisplayObject;
use crate::dive::display_objects::menu::MenuBar;
use crate::dive::display_objects::status::StatusBar;
use crate::dive::tab_manager::TabManager;

pub type AppRef = Rc<RefCell<App>>;

pub struct App {
    /// True when the application should exit
    pub should_quit: bool,

    /// List of display objects to render/handle
    pub display_objects: Vec<DisplayObject>,
    /// Index of the active display object (which handles key strokes)
    pub active_display_object_index: usize,

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

        let app = App {
            should_quit: false,

            display_objects: vec![],
            active_display_object_index: 0,

            status_bar: status_bar.clone(),
            menu_bar: menu_bar.clone(),
            tab_manager: tab_manager.clone(),
        };

        let app_ref = Rc::new(RefCell::new(app));

        let tab_display_obj= Rc::new(RefCell::new(dive::display_objects::tab_display::TabDisplay::new(tab_manager.clone())));

        // Add display objects
        app_ref.borrow_mut().display_objects.push(DisplayObject::new("menu", 128, menu_bar.clone(), true));
        app_ref.borrow_mut().display_objects.push(DisplayObject::new("status", 128, status_bar.clone(), true));
        app_ref.borrow_mut().display_objects.push(DisplayObject::new("tabs", 0, tab_display_obj.clone(), true));

        let test = Rc::new(RefCell::new(dive::display_objects::test::TestDisplayObject::new()));
        app_ref.borrow_mut().display_objects.push(DisplayObject::new("test", 64, test.clone(), false));

        let help = Rc::new(RefCell::new(dive::display_objects::help::HelpDisplayObject::new()));
        app_ref.borrow_mut().display_objects.push(DisplayObject::new("help", 0, help.clone(), false));

        app_ref
    }

    /// Find the display object with the given id or None when not found
    #[allow(dead_code)]
    pub(crate) fn find_display_object(&self, id: &str) -> Option<&DisplayObject> {
        for display_object in self.display_objects.iter() {
            if display_object.id == id {
                return Some(display_object);
            }
        }

        None
    }

    pub fn find_display_object_mut(&mut self, id: &str) -> Option<&mut DisplayObject> {
        for display_object in self.display_objects.iter_mut() {
            if display_object.id == id {
                return Some(display_object);
            }
        }

        None
    }

    /// Sets the given display object to be the active one (ie: handling events)
    pub(crate) fn set_active_display_object(&mut self, id: &str) {
        for (idx, display_object) in self.display_objects.iter_mut().enumerate() {
            if display_object.id == id {
                self.active_display_object_index = idx;
            }
        }
    }

    pub(crate) fn set_status(&mut self, status: &str) {
        self.status_bar.borrow_mut().set_status(status);
    }

    pub(crate) fn hide_display_object(&mut self, id: &str) {
        if let Some(display_object) = self.find_display_object_mut(id) {
            display_object.show = false;
        }
    }

    pub(crate) fn handle_events(&mut self, app: AppRef) -> anyhow::Result<()> {
        if ! event::poll(std::time::Duration::from_millis(250))? {
            return Ok(());
        }

        if let Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                let display_obj = &mut self.display_objects[self.active_display_object_index];

                let res = display_obj.object.borrow().event_handler(app, key)?;
                if res.is_none() {
                    // The display object did not handle the key, so we should handle it
                    self.process_key(key)?;
                }
            }
        }

        Ok(())
    }

    // Renders the screen, and all display objects
    pub(crate) fn render(&self, app: AppRef, f: &mut Frame) {
        // Iterate all display objects, and sort them by priority (0 == first)
        let mut objs = self.display_objects.clone();
        objs.sort_by(|a, b| a.priority.cmp(&b.priority));

        // Render all showable display objects
        for display_object in objs.iter_mut() {
            if !display_object.show {
                continue;
            }
            display_object.object.borrow_mut().render(app.clone(), f);
        }
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
                    self.status_bar.borrow_mut().set_status(format!("Switched to tab {}", digit).as_str());
                }
            },

            KeyCode::F(1) => {
                let obj = self.find_display_object_mut("help").unwrap();
                obj.show = !obj.show;

                self.set_active_display_object("help");
            }
            KeyCode::F(2) => {
                let obj = self.find_display_object_mut("test").unwrap();
                obj.show = !obj.show;

                self.set_active_display_object("test");
            }
            // KeyCode::F(9) => self.menu_active = !self.menu_active,
            KeyCode::Tab => {
                let idx;
                {
                    let mut tab_manager = self.tab_manager.borrow_mut();
                    idx = tab_manager.next();
                }
                self.set_status(format!("Switched to tab {}", idx).as_str());
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
                    self.set_status(format!("Closed tab {}", idx).as_str());
                } else {
                    self.set_status("Can't close last tab");
                }
            },
            Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                let idx;
                {
                    let mut tab_manager = self.tab_manager.borrow_mut();
                    idx = tab_manager.add_tab("New Tab", "gosub://blank");
                    tab_manager.switch(idx);
                }
                self.set_status(format!("Opened new tab {}", idx).as_str());
            },

            Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => self.should_quit = true,
            _ => {},
        }
        Ok(())
    }
}