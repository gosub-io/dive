use crate::dive::command_queue::CommandQueue;
use crate::dive::tab_manager::TabManager;
use crate::dive::ui::get_layout_chunks;
use crate::dive::widget_manager::Drawable;
use crossterm::event::KeyEvent;
use ratatui::layout::Layout;
use ratatui::prelude::{Constraint, Direction, Stylize};
use ratatui::widgets::{Block, Borders, Clear, ListState, Paragraph, Tabs, Wrap};
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
            tab_names.push(format!(
                " {} {}:{} ",
                if tab.secure { "🔒" } else { "" },
                idx,
                tab.name.clone()
            ));
        }

        let chunk = get_layout_chunks(f);

        let tab_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(0)].as_ref())
            .split(chunk[1]);

        let tabs = Tabs::new(tab_names)
            .block(Block::default().borders(Borders::NONE))
            .select(self.tab_manager.borrow().current)
            .divider("|")
            .padding("", "");

        f.render_widget(Clear, tab_layout[0]);
        f.render_widget(tabs, tab_layout[0]);

        let content = self.tab_manager.borrow().current().content.clone();
        let block = Block::default().borders(Borders::NONE).on_dark_gray();
        let paragraph = Paragraph::new(content)
            .block(block)
            .wrap(Wrap { trim: true });
        f.render_widget(Clear, tab_layout[1]);
        f.render_widget(paragraph, tab_layout[1]);
    }

    fn event_handler(
        &mut self,
        _queue: &mut CommandQueue,
        _key: KeyEvent,
    ) -> anyhow::Result<Option<KeyEvent>> {
        Ok(None)
    }
}
