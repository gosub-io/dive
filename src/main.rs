use std::cell::RefCell;
use std::rc::Rc;
use anyhow::Result;
use crossterm::{execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use ratatui::prelude::{CrosstermBackend, Terminal};
use crate::dive::app::App;
use crate::dive::widget_manager::Widget;
use crate::dive::widgets::splash::SplashWidget;

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
    let mut app = App::new();

    app.tab_manager.borrow_mut().open("New Tab", "gosub://blank");
    app.tab_manager.borrow_mut().open("Second Tab", "https://gosub.io");
    app.tab_manager.borrow_mut().open("Third Tab", "https://news.ycombinator.com");

    let w1 = Widget::new("splash", 255, false, Rc::new(RefCell::new(SplashWidget::new())));
    app.widget_manager.add(w1);
    app.widget_manager.show("splash", true);

    startup()?;
    let status = run(&mut app);
    shutdown()?;
    status?;
    Ok(())
}