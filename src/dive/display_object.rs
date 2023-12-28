use crossterm::event::KeyEvent;
use ratatui::Frame;
use crate::dive::app::App;

pub trait Displayable {
    fn render(&mut self, app: &mut App, f: &mut Frame);
    fn event_handler(&mut self, app: &mut App, key: KeyEvent) -> anyhow::Result<()>;
    fn on_show(&mut self, app: &mut App);
    fn on_hide(&mut self, app: &mut App);
}

pub struct DisplayObject {
    pub id: String,         // Unique identifier for this object
    pub priority: u8,       // 0 is the highest, 255 is the lowest
    pub show: bool,         // Does this object need to be rendered
    pub object: Box<dyn Displayable>,   // Actual object with rendering and event handling
}

impl DisplayObject {
    pub fn new(id: &str, priority: u8, object: Box<dyn Displayable>) -> Self {
        Self {
            id: id.into(),
            priority,
            show: false,
            object,
        }
    }

    pub fn show(&mut self, app: &mut App) {
        self.show = true;
        self.object.on_show(app);
    }

    pub fn hide(&mut self, app: &mut App) {
        self.show = false;
        self.object.on_hide(app);
    }
}
