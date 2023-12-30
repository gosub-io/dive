use std::cell::RefCell;
use std::rc::Rc;
use crossterm::event;
use crossterm::event::KeyCode::Char;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::event::Event::Key;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::Rect;
use crate::AppRef;
use crate::dive::tab::{Tab, tab_switch, tabs_render};
use crate::dive::display_object::DisplayObject;
use crate::dive::help::HelpDisplayObject;
use crate::dive::menu::MenuBar;
use crate::dive::status::StatusBar;
use crate::dive::test::TestDisplayObject;

pub struct App {
    pub tabs: Vec<Tab>,
    pub current_tab: usize,

    pub should_quit: bool,

    pub display_objects: Vec<DisplayObject>,
    pub active_display_object_index: usize,

    pub status_bar: Rc<RefCell<StatusBar>>,
    pub menu_bar: Rc<RefCell<MenuBar>>,
}

impl App {
    pub fn new() -> Self {
        let mut app = App {
            tabs: vec![],
            should_quit: false,
            current_tab: 0,

            display_objects: vec![],
            active_display_object_index: 0,

            status_bar: Rc::new(RefCell::new(StatusBar::new())),
            menu_bar: Rc::new(RefCell::new(MenuBar::new())),
        };

        let test = Rc::new(RefCell::new(TestDisplayObject::new()));
        let help = Rc::new(RefCell::new(HelpDisplayObject::new()));

        // Add display objects
        app.display_objects.push(DisplayObject::new("menu", 128, app.menu_bar.clone(), true));
        app.display_objects.push(DisplayObject::new("status", 128, app.status_bar.clone(), true));
        app.display_objects.push(DisplayObject::new("test", 64, test.clone(), true));
        app.display_objects.push(DisplayObject::new("help", 0, help.clone(), false));

        app
    }

    pub fn add_tab(&mut self, name: &str, url: &str) {
        let tab = Tab {
            name: name.into(),
            url: url.into(),
            content: String::new(),
        };

        self.tabs.push(tab);
    }

    /// Find the display object with the given id or None when not found
    pub(crate) fn find_display_object(&self, id: &str) -> Option<&DisplayObject> {
        for display_object in self.display_objects.iter() {
            if display_object.id == id {
                return Some(display_object);
            }
        }

        None
    }

    pub(crate) fn find_display_object_mut(&mut self, id: &str) -> Option<&mut DisplayObject> {
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

    pub(crate) fn handle_events(&mut self, app: AppRef) -> anyhow::Result<()> {
        if ! event::poll(std::time::Duration::from_millis(250))? {
            return Ok(());
        }

        if let Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                let display_obj = &mut self.display_objects[self.active_display_object_index];
                display_obj.object.borrow_mut().event_handler(app.clone(), key)?;

                // also let main app handle the key
                self.process_key(key)?;
            }
        }

        Ok(())
    }

    /// Returns the layout chunks for the main screen
    pub fn get_layout_chunks(&self, f: &mut Frame) -> Rc<[Rect]> {
        let size = f.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(1),      // menu bar
                    Constraint::Min(0),         // content
                    Constraint::Length(1),      // status bar
                ]
                    .as_ref(),
            )
            .split(size);

        chunks
    }

    // Renders the screen, and all display objects
    pub(crate) fn render(&mut self, app: AppRef, f: &mut Frame) {
        let chunks = self.get_layout_chunks(f);

        let tabs = tabs_render(self);
        f.render_widget(tabs, chunks[1]);

        // Iterate all display objects, and sort them by priority (0 == first)
        self.display_objects.sort_by(|a, b| a.priority.cmp(&b.priority));

        // Render all showable display objects
        for display_object in self.display_objects.iter_mut() {
            if !display_object.show {
                continue;
            }
            display_object.object.borrow_mut().render(app.clone(), f);
        }
    }

    /// Main key handling
    fn process_key(&mut self, key: KeyEvent) -> anyhow::Result<()> {
        match key.code {
            Char('0') if key.modifiers.contains(KeyModifiers::ALT) => tab_switch(self, 0),
            Char('1') if key.modifiers.contains(KeyModifiers::ALT) => tab_switch(self, 1),
            Char('2') if key.modifiers.contains(KeyModifiers::ALT) => tab_switch(self, 2),
            Char('3') if key.modifiers.contains(KeyModifiers::ALT) => tab_switch(self, 3),
            Char('4') if key.modifiers.contains(KeyModifiers::ALT) => tab_switch(self, 4),
            Char('5') if key.modifiers.contains(KeyModifiers::ALT) => tab_switch(self, 5),
            Char('6') if key.modifiers.contains(KeyModifiers::ALT) => tab_switch(self, 6),
            Char('7') if key.modifiers.contains(KeyModifiers::ALT) => tab_switch(self, 7),
            Char('8') if key.modifiers.contains(KeyModifiers::ALT) => tab_switch(self, 8),
            Char('9') if key.modifiers.contains(KeyModifiers::ALT) => tab_switch(self, 9),

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
                self.current_tab = (self.current_tab + 1) % self.tabs.len();
                self.set_status(format!("Switched to tab {}", self.current_tab).as_str());
            },

            // Char('i') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            //     // change the name of the current tab
            //     self.popup = true;
            // }
            Char('w') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                if self.tabs.len() > 1 {
                    self.tabs.remove(self.current_tab);
                    self.set_status(format!("Closed tab {}", self.current_tab).as_str());
                    if self.current_tab > 0 {
                        self.current_tab -= 1;
                    }
                } else {
                    self.set_status("Can't close last tab");
                }
            },
            Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.tabs.push(Tab {
                    name: "New Tab".to_string(),
                    url: "gosub://blank".to_string(),
                    content: String::new(),
                });
                self.set_status(format!("Opened new tab {}", self.tabs.len() - 1).as_str());
                self.current_tab = self.tabs.len() - 1;
            },
            Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => self.should_quit = true,
            _ => {},
        }
        Ok(())
    }
}