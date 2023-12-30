use std::cell::RefCell;
use std::rc::Rc;
use crossterm::event::KeyEvent;
use ratatui::Frame;
use crate::AppRef;

pub trait Displayable {
    fn render(&mut self, app: AppRef, f: &mut Frame);
    fn event_handler(&mut self, app: AppRef, key: KeyEvent) -> anyhow::Result<()>;
    fn on_show(&mut self, app: AppRef);
    fn on_hide(&mut self, app: AppRef);
}

pub struct DisplayObject {
    pub id: String,         // Unique identifier for this object
    pub priority: u8,       // 0 is the highest, 255 is the lowest
    pub show: bool,         // Does this object need to be rendered
    pub object: Rc<RefCell<dyn Displayable>>,   // Actual object with rendering and event handling
}

impl DisplayObject {
    pub fn new(id: &str, priority: u8, object: Rc<RefCell<dyn Displayable>>, show: bool) -> Self {
        Self {
            id: id.into(),
            priority,
            show,
            object,
        }
    }

    pub fn show(&mut self, app: AppRef) {
        self.show = true;
        self.object.borrow_mut().on_show(app.clone());
    }

    pub fn hide(&mut self, app: AppRef) {
        self.show = false;
        self.object.borrow_mut().on_hide(app.clone());
    }
}
