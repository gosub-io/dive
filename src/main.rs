use anyhow::Result;
use crossterm::{execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use ratatui::prelude::{CrosstermBackend, Terminal};
use crate::dive::app::App;
use simple_logger::SimpleLogger;

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

fn run(app: &mut App) -> Result<()> {
    let mut t = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;
    let _ = t.clear()?;

    loop {
        t.draw(|f| {
            app.widget_manager.render(f);
        })?;

        app.handle_events()?;
        app.process_commands();

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    // SimpleLogger::new().init().unwrap();

    let mut app = App::new();

    app.tab_manager.borrow_mut().open("New Tab", "gosub://blank");
    app.tab_manager.borrow_mut().open("Second Tab", "https://gosub.io");
    app.tab_manager.borrow_mut().open("Third Tab", "https://news.ycombinator.com");

    startup()?;
    let status = run(&mut app);
    shutdown()?;
    status?;
    Ok(())
}