use anyhow::Result;
use crossterm::{execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use ratatui::prelude::{CrosstermBackend, Terminal};
use crate::dive::app::App;

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
            app.widget_manager.render(&mut app, f);
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

// fn handle_events(app: &mut App) -> Result<()> {
//     if event::poll(std::time::Duration::from_millis(100))? {
//         if let event::Event::Key(key) = event::read()? {
//             app.handle_event(key)?;
//         }
//     }
//
//     Ok(())
// }
//
// fn render(app: &mut App, f: &mut Frame) {
//     let mut objs = Vec::new();
//
//     // Fetch all visible objects
//     let binding = app.borrow();
//     let binding = binding.obj_manager.borrow();
//     for display_object in binding.objects.iter() {
//         if display_object.visible {
//             objs.push(display_object);
//         }
//     }
//
//     // Render all visible display objects
//     for display_object in objs.iter() {
//         display_object.inner.borrow_mut().render(app.clone(), f);
//     }
// }

fn main() -> Result<()> {
    let mut app = App::new();

    app.tab_manager.open("New Tab", "gosub://blank");
    app.tab_manager.open("Second Tab", "https://gosub.io");
    app.tab_manager.open("Third Tab", "https://news.ycombinator.com");

    startup()?;
    let status = run(&mut app);
    shutdown()?;
    status?;
    Ok(())
}