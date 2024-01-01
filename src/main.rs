use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::{CrosstermBackend, Terminal};
use crate::dive::app::{App, AppRef};

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

fn run(app: AppRef) -> anyhow::Result<()> {
    let mut t = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    loop {
        t.draw(|f| {
            app.borrow().render(app.clone(), f);
        })?;

        // application update
        app.borrow().handle_events(app.clone())?;

        // application exit
        if app.borrow().should_quit {
            break;
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let app = App::new();

    app.borrow_mut().tab_manager.borrow_mut().add_tab("New Tab", "gosub://blank");
    app.borrow_mut().tab_manager.borrow_mut().add_tab("Second Tab", "https://gosub.io");
    app.borrow_mut().tab_manager.borrow_mut().add_tab("Third Tab", "https://news.ycombinator.com");

    startup()?;
    let status = run(app.clone());
    shutdown()?;
    status?;
    Ok(())
}

