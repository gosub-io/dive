use crossterm::event;
use crossterm::event::KeyCode::Char;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::event::Event::Key;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use crate::dive::help::{help_process_keys, help_render};
use crate::dive::menu::menu_render;
use crate::dive::status::status_render;
use crate::dive::tab::{Tab, tab_switch, tabs_render};

pub struct App {
    pub tabs: Vec<Tab>,
    pub should_quit: bool,
    pub menu_active: bool,
    pub menu_item_active: usize,
    pub current_tab: usize,
    pub show_help: bool,
    pub help_scroll: u16,
    pub status: String,
}

pub fn app_ui(app: &mut App, f: &mut Frame) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(1),      // menu bar
                Constraint::Min(0),         // content
                Constraint::Length(1),      // status bar
            ]
                .as_ref(),
        )
        .split(size);

    let menu = menu_render(app);
    f.render_widget(menu, chunks[0]);

    let tabs = tabs_render(app);
    f.render_widget(tabs, chunks[1]);

    let status = status_render(app);
    f.render_widget(status, chunks[2]);

    if app.show_help {
        help_render(app, f);
    }
}

fn main_process_keys(app: &mut App, key: KeyEvent) -> anyhow::Result<()> {
    match key.code {
        Char('0') if key.modifiers.contains(KeyModifiers::ALT) => tab_switch(app, 0),
        Char('1') if key.modifiers.contains(KeyModifiers::ALT) => tab_switch(app, 1),
        Char('2') if key.modifiers.contains(KeyModifiers::ALT) => tab_switch(app, 2),
        Char('3') if key.modifiers.contains(KeyModifiers::ALT) => tab_switch(app, 3),
        Char('4') if key.modifiers.contains(KeyModifiers::ALT) => tab_switch(app, 4),
        Char('5') if key.modifiers.contains(KeyModifiers::ALT) => tab_switch(app, 5),
        Char('6') if key.modifiers.contains(KeyModifiers::ALT) => tab_switch(app, 6),
        Char('7') if key.modifiers.contains(KeyModifiers::ALT) => tab_switch(app, 7),
        Char('8') if key.modifiers.contains(KeyModifiers::ALT) => tab_switch(app, 8),
        Char('9') if key.modifiers.contains(KeyModifiers::ALT) => tab_switch(app, 9),

        KeyCode::F(1) => {
            app.show_help = !app.show_help;
            app.help_scroll = 0;
            if app.show_help {
                app.status = "Opened help screen".into();
            } else {
                app.status = "Closed help screen".into();
            }
        }
        KeyCode::F(9) => app.menu_active = !app.menu_active,
        KeyCode::Tab => {
            app.current_tab = (app.current_tab + 1) % app.tabs.len();
            app.status = format!("Switched to tab {}", app.current_tab);
        },

        Char('w') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            if app.tabs.len() > 1 {
                app.tabs.remove(app.current_tab);
                app.status = format!("Closed tab {}", app.current_tab);
                if app.current_tab > 0 {
                    app.current_tab -= 1;
                }
            } else {
                app.status = "Can't close last tab".into();
            }
        },
        Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.tabs.push(Tab {
                name: "New Tab".to_string(),
                url: "gosub://blank".to_string(),
                content: String::new(),
            });
            app.status = format!("Opened new tab {}", app.tabs.len() - 1);
            app.current_tab = app.tabs.len() - 1;
        },
        Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => app.should_quit = true,
        _ => {},
    }
    Ok(())
}

pub fn app_update(app: &mut App) -> anyhow::Result<()> {
    if ! event::poll(std::time::Duration::from_millis(250))? {
        return Ok(());
    }

    if let Key(key) = event::read()? {
        if key.kind == event::KeyEventKind::Press {
            if app.show_help {
                help_process_keys(app, key)?;
            } else {
                main_process_keys(app, key)?;
            }
        }
    }

    Ok(())
}