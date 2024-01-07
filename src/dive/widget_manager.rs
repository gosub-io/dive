use crate::dive::command_queue::CommandQueue;
use crossterm::event::KeyEvent;
use ratatui::Frame;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Drawable {
    fn render(&mut self, f: &mut Frame);
    fn event_handler(
        &mut self,
        queue: &mut CommandQueue,
        key: KeyEvent,
    ) -> anyhow::Result<Option<KeyEvent>>;
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
    pub inner: Rc<RefCell<dyn Drawable>>,
}

impl Widget {
    pub fn new(id: &str, priority: u8, visible: bool, inner: Rc<RefCell<dyn Drawable>>) -> Self {
        Self {
            id: id.into(),
            priority,
            visible,
            inner,
        }
    }

    pub fn render(&mut self, f: &mut Frame) {
        if self.visible {
            self.inner.borrow_mut().render(f);
        }
    }
}

pub struct WidgetManager {
    pub widgets: Vec<Widget>,
    pub focussed_widget_id: Option<String>,
}

impl WidgetManager {
    pub fn new() -> Self {
        Self {
            widgets: vec![],
            focussed_widget_id: None,
        }
    }

    pub(crate) fn destroy(&mut self, id: &String) {
        if let Some(index) = self.widgets.iter().position(|w| w.id == *id) {
            self.widgets.remove(index);
        }
    }

    pub fn create(&mut self, widget: Widget) {
        self.widgets.push(widget);
    }

    #[allow(dead_code)]
    pub fn find(&mut self, id: &str) -> Option<&mut Widget> {
        self.widgets.iter_mut().find(|widget| widget.id == id)
    }

    pub(crate) fn focussed(&self) -> Option<&Widget> {
        let focus_id = match self.focussed_widget_id {
            Some(ref id) => id.clone(),
            None => return None,
        };

        self.widgets.iter().find(|&widget| widget.id == focus_id)
    }

    pub(crate) fn render(&mut self, f: &mut Frame) {
        for widget in &mut self.widgets {
            widget.render(f);
        }
    }

    pub(crate) fn show(&mut self, id: &str, focus: bool) {
        for widget in &mut self.widgets {
            if widget.id == id {
                widget.visible = true;

                if focus {
                    self.focussed_widget_id = Some(id.into());
                }
            }
        }
    }

    pub(crate) fn hide(&mut self, id: &str) {
        for widget in &mut self.widgets {
            if widget.id == id {
                widget.visible = false;

                // remove focus if we had it
                if self.focussed_widget_id == Some(id.into()) {
                    self.focussed_widget_id = None;
                }
            }
        }
    }

    pub(crate) fn toggle(&mut self, id: &str, focus: bool) {
        for widget in &mut self.widgets {
            if widget.id == id {
                widget.visible = !widget.visible;

                // Remove focus when hiding and it was the focussed widget
                if !widget.visible && self.focussed_widget_id == Some(id.into()) {
                    self.focussed_widget_id = None;
                }

                if focus && widget.visible {
                    self.focussed_widget_id = Some(id.into());
                }
            }
        }
    }

    #[allow(dead_code)]
    pub(crate) fn is_visible(&self, id: &str) -> bool {
        for widget in &self.widgets {
            if widget.id == id {
                return widget.visible;
            }
        }

        false
    }
}
