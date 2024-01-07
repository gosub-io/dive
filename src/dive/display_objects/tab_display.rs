use crate::dive::app::AppRef;
use crate::dive::obj_manager::Displayable;
use crate::dive::tab_manager::TabManager;
use crate::dive::ui::get_layout_chunks;
use crossterm::event::KeyEvent;
use ratatui::widgets::{Block, Borders, Clear, Tabs};
use ratatui::Frame;
use std::cell::RefCell;
use std::rc::Rc;

pub struct TabDisplay {
    pub manager: Rc<RefCell<TabManager>>,
}

impl TabDisplay {
    pub fn new(manager: Rc<RefCell<TabManager>>) -> Self {
        Self { manager }
    }
}

impl Displayable for TabDisplay {
    fn render(&mut self, _app: AppRef, f: &mut Frame) {
        let mut tab_names = Vec::new();
        for (idx, tab) in self.manager.borrow().tabs.iter().enumerate() {
            tab_names.push(format!(" {}:{} ", idx, tab.name.clone()));
        }

        let tabs = Tabs::new(tab_names)
            .block(Block::default().borders(Borders::NONE))
            // .style(Style::default().white())
            // .highlight_style(Style::default().yellow())
            .select(self.manager.borrow().current)
            .divider("|")
            .padding("", "");

        let chunk = get_layout_chunks(f);
        f.render_widget(Clear, chunk[1]);
        f.render_widget(tabs, chunk[1]);
    }

    fn event_handler(&mut self, _app: AppRef, _key: KeyEvent) -> anyhow::Result<Option<KeyEvent>> {
        Ok(None)
    }

    fn on_show(&mut self, _app: AppRef) {}

    fn on_hide(&mut self, _app: AppRef) {}
}
