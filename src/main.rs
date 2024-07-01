use crate::dive::app::App;
use crate::dive::gosub_logger::{GosubLogger, LogPool};
use crate::dive::widget_manager::Widget;
use crate::dive::widgets::splash::SplashWidget;
use anyhow::Result;
use better_panic::Settings;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::LevelFilter;
use ratatui::prelude::{CrosstermBackend, Terminal};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

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
    t.clear()?;

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

pub fn initialize_panic_handler() {
    std::panic::set_hook(Box::new(|panic_info| {
        crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen).unwrap();
        crossterm::terminal::disable_raw_mode().unwrap();
        Settings::auto()
            .most_recent_first(false)
            .lineno_suffix(true)
            .create_panic_handler()(panic_info);
    }));
}

fn main() -> Result<()> {
    let log_pool = Arc::new(Mutex::new(LogPool::new(10)));

    let logger = GosubLogger::new(log_pool.clone());
    log::set_max_level(LevelFilter::Trace);
    let _ = log::set_boxed_logger(Box::new(logger));
    log::error!("Starting Gosub...");
    log::warn!("Starting Gosub...");
    log::info!("Starting Gosub...");
    log::trace!("Starting Gosub...");
    log::debug!("Starting Gosub...");

    let mut app = App::new(log_pool);

    app.tab_manager
        .borrow_mut()
        .open("New Tab", "gosub://blank");

    let w1 = Widget::new(
        "splash",
        255,
        false,
        Rc::new(RefCell::new(SplashWidget::new())),
    );
    app.widget_manager.create(w1);
    app.widget_manager.show("splash", true);

    startup()?;
    let status = run(&mut app);
    shutdown()?;
    status?;
    Ok(())
}
