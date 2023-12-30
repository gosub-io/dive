use ratatui::widgets::{Block, Borders, Tabs};
use crate::dive::app::App;

pub struct Tab {
    pub name: String,
    pub url: String,
    pub content: String,
}

pub fn tab_switch(app: &mut App, tab: usize) {
    if tab < app.tabs.len() {
        app.current_tab = tab;
        app.set_status(format!("Switched to tab {}", app.current_tab).as_str());
    }
}

pub fn tabs_render(app: &mut App) -> Tabs<'static> {
    let mut tab_names = Vec::new();
    for (idx, tab) in app.tabs.iter().enumerate() {
        tab_names.push(format!(" {}:{} ", idx, tab.name.clone()));
    }

    Tabs::new(tab_names)
        .block(Block::default().borders(Borders::NONE))
        // .style(Style::default().white())
        // .highlight_style(Style::default().yellow())
        .select(app.current_tab)
        .divider("|")
        .padding("", "")
}