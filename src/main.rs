use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::{CrosstermBackend, Terminal};
use crate::dive::app::App;
use crate::dive::tab::Tab;

mod dive;

fn startup() -> Result<()> {
    enable_raw_mode()?;
    execute!(std::io::stderr(), EnterAlternateScreen)?;
    Ok(())
}

fn shutdown() -> Result<()> {
    execute!(std::io::stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn run(app: &mut App) -> anyhow::Result<()> {
    let mut t = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    loop {
        t.draw(|f| {
            app.render(f);
        })?;

        // application update
        app.handle_events()?;

        // application exit
        if app.should_quit {
            break;
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let tab1 = Tab {
        name: "New Tab".to_string(),
        url: "gosub://blank".to_string(),
        content: String::new(),
    };

    let mut app = App {
        tabs: vec![tab1],
        should_quit: false,
        menu_active: false,
        menu_item_active: 0,
        current_tab: 0,
        // show_help: false,
        status: "Press F1 for help".into(),
        // vertical_scroll_state: Default::default(),
        // vertical_scroll: 0,
        // vertical_scroll_max: 0,
        // popup: true,

        display_objects: vec![],
        active_display_object_index: 0,
    };

    startup()?;
    let status = run(&mut app);
    shutdown()?;
    status?;
    Ok(())
}

