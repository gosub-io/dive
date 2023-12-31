use std::cell::RefCell;
use std::rc::Rc;
use crossterm::event::KeyEvent;
use ratatui::Frame;
use crate::dive::app::AppRef;

pub trait Displayable {
    fn render(&mut self, app: AppRef, f: &mut Frame);
    fn event_handler(&mut self, app: AppRef, key: KeyEvent) -> anyhow::Result<Option<KeyEvent>>;
    fn on_show(&mut self, app: AppRef);
    fn on_hide(&mut self, app: AppRef);
}

#[derive(Clone)]
pub struct DisplayObject {
    /// Unique identifier for this object
    pub id: String,
    /// 0 is the highest, 255 is the lowest
    pub priority: u8,
    /// Does this object need to be rendered
    pub show: bool,
    /// Actual object with rendering and event handling
    pub object: Rc<RefCell<dyn Displayable>>,
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
}
