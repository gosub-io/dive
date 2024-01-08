use crate::dive::command_queue::CommandQueue;
use crate::dive::tab_manager::TabManager;
use crate::dive::ui::get_layout_chunks;
use crate::dive::widget_manager::Drawable;
use crossterm::event::KeyEvent;
use ratatui::widgets::{Block, Borders, Clear, ListState, Tabs};
use ratatui::Frame;
use std::cell::RefCell;
use std::rc::Rc;

pub struct TabsWidget {
    pub tab_manager: Rc<RefCell<TabManager>>,
    pub state: ListState,
}

impl TabsWidget {
    pub fn new(tab_manager: Rc<RefCell<TabManager>>) -> Self {
        let tm = tab_manager.clone();
        let widget = Self {
            tab_manager: tm.clone(),
            state: ListState::default().with_selected(Some(tm.borrow().current)),
        };

        widget
    }
}

impl Drawable for TabsWidget {
    fn on_show(&mut self) {}
    fn on_hide(&mut self) {}

    fn render(&mut self, f: &mut Frame) {
        let mut tab_names = Vec::new();
        for (idx, tab) in self.tab_manager.borrow_mut().tabs.iter().enumerate() {
            tab_names.push(format!(" {}:{} ", idx, tab.name.clone()));
        }

        let tabs = Tabs::new(tab_names)
            .block(Block::default().borders(Borders::NONE))
            .select(self.tab_manager.borrow().current)
            .divider("|")
            .padding("", "");

        let chunk = get_layout_chunks(f);
        f.render_widget(Clear, chunk[1]);
        f.render_widget(tabs, chunk[1]);
    }

    fn event_handler(
        &mut self,
        _queue: &mut CommandQueue,
        _key: KeyEvent,
    ) -> anyhow::Result<Option<KeyEvent>> {
        Ok(None)
    }
}
