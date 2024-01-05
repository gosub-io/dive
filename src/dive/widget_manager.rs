use ratatui::Frame;
use crate::dive::app::App;

pub trait Drawable {
    fn render(&mut self, app: &mut App, f: &mut Frame);
    // fn event_handler(&mut self, app: &mut App, key: KeyEvent) -> anyhow::Result<Option<KeyEvent>>;
    // fn on_show(&mut self, app: &mut App);
    // fn on_hide(&mut self, app: &mut App);
}

pub struct Widget {
    /// Unique identifier for this widget
    pub id: String,
    /// 0 is the highest, 255 is the lowest
    pub priority: u8,
    /// Does this object need to be rendered
    pub visible: bool,
    /// Actual object with rendering and event handling
    pub inner: Box<dyn Drawable>,
}

impl Widget {
    pub fn new(id: &str, priority: u8, visible: bool, inner: Box<dyn Drawable>) -> Self {
        Self {
            id: id.into(),
            priority,
            visible,
            inner,
        }
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }

    pub fn show(&mut self) {
        self.visible = true;
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    pub fn priority(&mut self, priority: u8) {
        self.priority = priority;
    }

    pub fn render(&mut self, app: &mut App, f: &mut Frame) {
        if self.visible {
            self.inner.render(app, f);
        }
    }
}

pub struct WidgetManager {
    pub widgets: Vec<Widget>,
    pub focus: Option<usize>,
}

impl WidgetManager {
    pub fn new() -> Self {
        Self {
            widgets: vec![],
            focus: None,
        }
    }

    pub fn add(&mut self, widget: Widget) {
        self.widgets.push(widget);
        // self.widgets.push(Box::new(move |app: &mut App, f: &mut Frame| widget.render(app, f)));
        // self.widgets.sort_by(|a, b| a.priority.cmp(&b.priority));
    }

    // #[allow(dead_code)]
    // pub fn active(&self) -> Option<&Widget<T>> {
    //     if self.focus.is_none() {
    //         return None;
    //     }
    //
    //     Some(&self.widgets[self.focus.unwrap()])
    // }

    pub fn find(&mut self, id: &str) -> Option<&mut Widget> {
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

    pub(crate) fn render(&mut self, app: &mut App, f: &mut Frame) {
        for widget in &mut self.widgets {
            widget.render(app, f);
        }
    }
}