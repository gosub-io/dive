use crate::dive::command_queue::{Command, CommandQueue};
use crate::dive::ui::centered_rect;
use crate::dive::widget_manager::Drawable;
use crate::dive::widgets::tab_manager::TabManager;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Clear, List, ListDirection, ListState, Padding};
use ratatui::Frame;
use std::cell::RefCell;
use std::rc::Rc;

pub struct TabListWidget {
    pub tab_manager: Rc<RefCell<TabManager>>,
    pub state: ListState,
}

impl TabListWidget {
    pub fn new(tab_manager: Rc<RefCell<TabManager>>) -> Self {
        let tm = tab_manager.clone();
        let widget = Self {
            tab_manager: tm.clone(),
            state: ListState::default().with_selected(Some(tm.borrow().current)),
        };

        widget
    }
}

impl Drawable for TabListWidget {
    fn render(&mut self, f: &mut Frame) {
        let mut items = vec![];
        for (idx, tab) in self.tab_manager.borrow().tabs.iter().enumerate() {
            items.push(Span::styled(
                format!("{}) {} - {}", idx, tab.name.clone(), tab.url.clone()),
                Style::default().fg(Color::White),
            ));
        }

        let block = Block::default()
            .title("Tab List")
            .borders(Borders::ALL)
            .padding(Padding::new(1, 1, 1, 1));

        let list = List::new(items)
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default()
                    .bg(Color::Red)
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom)
            .block(block);

        let area = centered_rect(60, 25, f.size());
        f.render_widget(Clear, area);

        f.render_stateful_widget(list, area, &mut self.state);
    }

    fn event_handler(
        &mut self,
        queue: &mut CommandQueue,
        key: KeyEvent,
    ) -> anyhow::Result<Option<KeyEvent>> {
        match key.code {
            KeyCode::Esc => {
                queue.push(Command::DestroyWidget {
                    id: "tab_list".into(),
                });
            }
            KeyCode::Down => {
                let mut sel = self.state.selected().unwrap_or(0);
                if sel < self.tab_manager.borrow().tabs.len() - 1 {
                    sel += 1;
                }
                self.state = self.state.clone().with_selected(Some(sel));
            }
            KeyCode::Up => {
                let mut sel = self.state.selected().unwrap_or(0);
                sel = sel.saturating_sub(1);
                self.state = self.state.clone().with_selected(Some(sel));
            }
            KeyCode::Enter => {
                let sel = self.state.selected().unwrap_or(0);
                self.tab_manager.borrow_mut().switch(sel);
                queue.push(Command::DestroyWidget {
                    id: "tab_list".into(),
                });
            }
            _ => {}
        }

        Ok(Some(key))
    }
}
