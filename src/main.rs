use anyhow::Result;
use crossterm::{event, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use crossterm::event::Event::Key;
use ratatui::Frame;
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

fn run(app: AppRef) -> Result<()> {
    let mut t = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;
    let _ = t.clear()?;

    loop {
        t.draw(|f| {
            render(app.clone(), f);
        })?;

        // application update
        handle_events(app.clone())?;

        // application exit
        if app.borrow().vars.should_quit {
            break;
        }
    }

    Ok(())
}

fn handle_events(app: AppRef) -> Result<()> {
    if ! event::poll(std::time::Duration::from_millis(250))? {
        return Ok(());
    }

    if let Key(key) = event::read()? {
        if key.kind != event::KeyEventKind::Press {
            return Ok(())
        }

        let res;
        {
            let binding = app.borrow();
            let obj_manager = binding.obj_manager.borrow();

            let active = obj_manager.active().clone();
            match active {
                Some(active) => {
                    res = active.inner.borrow_mut().event_handler(app.clone(), key)?;
                },
                None => {
                    res = None;
                }
            }
        }

        if res.is_none() {
            // The display object did not handle the key, so we should handle it
            app.borrow_mut().process_key(app.clone(), key)?;
        }
    }

    Ok(())
}

fn render(app: AppRef, f: &mut Frame) {
    let mut objs = Vec::new();

    // Fetch all visible objects
    let binding = app.borrow();
    let binding = binding.obj_manager.borrow();
    for display_object in binding.objects.iter() {
        if display_object.visible {
            objs.push(display_object);
        }
    }

    // Render all visible display objects
    for display_object in objs.iter() {
        display_object.inner.borrow_mut().render(app.clone(), f);
    }
}

fn main() -> Result<()> {
    let app = App::new();

    app.borrow().tab_manager.borrow_mut().add_tab("New Tab", "gosub://blank");
    app.borrow().tab_manager.borrow_mut().add_tab("Second Tab", "https://gosub.io");
    app.borrow().tab_manager.borrow_mut().add_tab("Third Tab", "https://news.ycombinator.com");

    startup()?;
    let status = run(app.clone());
    shutdown()?;
    status?;
    Ok(())
}

