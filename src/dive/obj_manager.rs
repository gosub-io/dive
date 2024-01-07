use crate::dive::app::AppRef;
use crossterm::event::KeyEvent;
use ratatui::Frame;
use std::cell::RefCell;
use std::rc::Rc;

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
    pub visible: bool,
    /// Actual object with rendering and event handling
    pub inner: Rc<RefCell<dyn Displayable>>,
}

impl DisplayObject {
    pub fn new(id: &str, priority: u8, inner: Rc<RefCell<dyn Displayable>>, visible: bool) -> Self {
        Self {
            id: id.into(),
            priority,
            visible,
            inner,
        }
    }
}

pub struct DisplayObjectManager {
    pub objects: Vec<DisplayObject>,
    active_display_object_index: Option<usize>,
}

impl DisplayObjectManager {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            active_display_object_index: None,
        }
    }

    pub fn add(&mut self, display_object: DisplayObject) {
        self.objects.push(display_object);

        self.objects.sort_by(|a, b| a.priority.cmp(&b.priority));
    }

    pub fn active(&self) -> Option<&DisplayObject> {
        self.active_display_object_index?;

        Some(&self.objects[self.active_display_object_index.unwrap()])
    }

    pub fn find(&mut self, id: &str) -> Option<&mut DisplayObject> {
        for display_object in self.objects.iter_mut() {
            if display_object.id == id {
                return Some(display_object);
            }
        }

        None
    }

    /// Sets the given display object to be the active one (ie: handling events)
    pub(crate) fn activate(&mut self, id: &str) {
        for (idx, display_object) in self.objects.iter_mut().enumerate() {
            if display_object.id == id {
                self.active_display_object_index = Some(idx);
            }
        }
    }

    pub(crate) fn deactivate(&mut self) {
        self.active_display_object_index = None;
    }

    pub(crate) fn visible(&mut self, id: &str, visibility: bool) {
        if let Some(display_object) = self.find(id) {
            display_object.visible = visibility;
        }
    }

    pub(crate) fn toggle_visible(&mut self, id: &str) {
        if let Some(display_object) = self.find(id) {
            display_object.visible = !display_object.visible;
        }
    }
}
