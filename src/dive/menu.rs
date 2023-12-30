use ratatui::prelude::*;
use ratatui::widgets::Paragraph;
use crossterm::event::KeyEvent;
use crate::AppRef;
use crate::dive::display_object::Displayable;


pub struct MenuBar {
    pub active: bool,
    pub menu_item_active: u8,
}

impl MenuBar {
    pub fn new() -> Self {
        Self {
            active: false,
            menu_item_active: 0,
        }
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }
}

impl Displayable for MenuBar {
    fn render(&mut self, app: AppRef, f: &mut Frame) {

        let menu_items = vec![
            "File",
            "Edit",
            "View",
            "History",
            "Bookmarks",
            "Tools",
            "Help",
        ];

        let mut menu_tiles = vec![
            Span::styled(" Gosub Dive ", Style::default().fg(Color::White).bold()),
        ];

        for (idx, item) in menu_items.iter().enumerate() {
            menu_tiles.push(Span::raw("|"));

            if self.active && self.menu_item_active == idx as u8 {
                menu_tiles.push(Span::styled(
                    format!(" {} ", item),
                    Style::default().bg(Color::Green).fg(Color::White).add_modifier(Modifier::BOLD),
                ));
            } else {
                menu_tiles.push(Span::raw(format!(" {} ", item)));
            }
        }

        let chunks = app.borrow().get_layout_chunks(f);
        let menu_bar = Paragraph::new(Line::from(menu_tiles)).style(Style::default().bg(Color::Blue).add_modifier(Modifier::BOLD));
        f.render_widget(menu_bar, chunks[0]);
    }

    fn event_handler(&mut self, _app: AppRef, _key: KeyEvent) -> anyhow::Result<()> {
        // We should handle left, right to change the menu item active
        Ok(())
    }

    fn on_show(&mut self, _app: AppRef) { }

    fn on_hide(&mut self, _app: AppRef) { }
}