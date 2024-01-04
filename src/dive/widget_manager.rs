use crossterm::event::KeyEvent;
use ratatui::Frame;
use crate::dive::app::App;

pub trait Widget {
    fn render(&mut self, app: &mut App, f: &mut Frame);
    fn event_handler(&mut self, app: &mut App, key: KeyEvent) -> anyhow::Result<Option<KeyEvent>>;
    fn on_show(&mut self, app: &mut App);
    fn on_hide(&mut self, app: &mut App);
}

pub struct WidgetObject {
    /// Unique identifier for this widget
    pub id: String,
    /// 0 is the highest, 255 is the lowest
    pub priority: u8,
    /// Does this object need to be rendered
    pub visible: bool,
    /// Actual object with rendering and event handling
    pub inner: Box<dyn Widget>,
}

impl WidgetObject {
    pub fn new(id: &str, priority: u8, inner: Box<dyn Widget>, visible: bool) -> Self {
        Self {
            id: id.into(),
            priority,
            visible,
            inner,
        }
    }
}

pub struct WidgetManager {
    pub widgets: Vec<WidgetObject>,
    pub focus: Option<usize>,
}

impl WidgetManager {
    pub fn new() -> Self {
        Self {
            widgets: vec![],
            focus: None,
        }
    }

    pub fn add(&mut self, widget: WidgetObject) {
        self.widgets.push(widget);

        self.widgets.sort_by(|a, b| a.priority.cmp(&b.priority));
    }

    #[allow(dead_code)]
    pub fn active(&self) -> Option<&WidgetObject> {
        if self.focus.is_none() {
            return None;
        }

        Some(&self.widgets[self.focus.unwrap()])
    }

    pub fn find(&mut self, id: &str) -> Option<&mut WidgetObject> {
        for widget_object in self.widgets.iter_mut() {
            if widget_object.id == id {
                return Some(widget_object);
            }
        }

        None
    }

    /// Sets the given display object to be the active one (ie: handling events)
    pub(crate) fn focus(&mut self, id: &str) {
        for (idx, widget_object) in self.widgets.iter_mut().enumerate() {
            if widget_object.id == id {
                self.focus = Some(idx);
            }
        }
    }

    pub(crate) fn unfocus(&mut self, _id: &str) {
        self.focus = None;
    }

    pub(crate) fn show(&mut self, id: &str) {
        if let Some(widget_object) = self.find(id) {
            widget_object.visible = true;
        }
    }

    pub(crate) fn hide(&mut self, id: &str) {
        if let Some(widget_object) = self.find(id) {
            widget_object.visible = true;
        }
    }

    pub(crate) fn toggle(&mut self, id:&str) {
        if let Some(widget_object) = self.find(id) {
            widget_object.visible = !widget_object.visible;
        }
    }

    pub(crate) fn render(&mut self, app: &mut App, f: &mut Frame) {
        let mut objs = vec![];
        for widget_object in self.widgets.iter() {
            if widget_object.visible {
                objs.push(widget_object);
            }
        }

        for widget_object in objs.iter_mut() {
            widget_object.inner.render(app, f);
        }
    }
}