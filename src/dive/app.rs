use crate::dive::bookmark_manager::BookmarkManager;
use crate::dive::command_queue::{Command, CommandQueue};
use crate::dive::tab_manager::TabManager;
use crate::dive::widget_manager::{Widget, WidgetManager};
use crate::dive::widgets::bookmark_list::BookmarkListWidget;
use crate::dive::widgets::help::Help;
use crate::dive::widgets::input::{InputSubmitCommand, InputWidget};
use crate::dive::widgets::menu_bar::MenuBar;
use crate::dive::widgets::status_bar::StatusBar;
use crate::dive::widgets::tab_list::TabListWidget;
use crate::dive::widgets::tabs::TabsWidget;
use crossterm::event;
use crossterm::event::Event::Key;
use crossterm::event::KeyCode::Char;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::cell::RefCell;
use std::rc::Rc;

pub struct App {
    pub should_quit: bool,

    pub status_bar: Rc<RefCell<StatusBar>>,
    pub menu_bar: Rc<RefCell<MenuBar>>,
    pub tab_manager: Rc<RefCell<TabManager>>,
    pub bookmark_manager: Rc<RefCell<BookmarkManager>>,

    pub widget_manager: WidgetManager,
    pub command_queue: CommandQueue,
}

impl App {
    pub fn new() -> Self {
        let bm = BookmarkManager::new_from_file("bookmarks.json");

        let mut app = Self {
            should_quit: false,

            status_bar: Rc::new(RefCell::new(StatusBar::new())),
            menu_bar: Rc::new(RefCell::new(MenuBar::new())),
            tab_manager: Rc::new(RefCell::new(TabManager::new())),
            bookmark_manager: Rc::new(RefCell::new(bm)),

            widget_manager: WidgetManager::new(),
            command_queue: CommandQueue::new(),
        };

        // Add the main widgets
        let w1 = Widget::new("statusbar", 0, true, app.status_bar.clone());
        app.widget_manager.create(w1);
        let w1 = Widget::new("menubar", 0, true, app.menu_bar.clone());
        app.widget_manager.create(w1);

        let inner = TabsWidget::new(app.tab_manager.clone());
        let w1 = Widget::new("tabs", 0, true, Rc::new(RefCell::new(inner)));
        app.widget_manager.create(w1);

        app
    }

    pub(crate) fn handle_events(&mut self) -> anyhow::Result<()> {
        if !event::poll(std::time::Duration::from_millis(250))? {
            return Ok(());
        }

        if let Key(key) = event::read()? {
            if key.kind != event::KeyEventKind::Press {
                return Ok(());
            }

            let mut handle_as_unfocussed = true;
            if let Some(widget) = self.widget_manager.focussed() {
                if let Ok(Some(_)) = widget
                    .inner
                    .borrow_mut()
                    .event_handler(&mut self.command_queue, key)
                {
                    handle_as_unfocussed = false;
                }
            }

            if handle_as_unfocussed {
                self.process_key(key)?;
            }

            // self.widget_manager.find("help").unwrap().inner.event_handler(&mut self.command_queue, key)?;
        }

        Ok(())
    }

    /// Main key handling
    fn process_key(&mut self, key: KeyEvent) -> anyhow::Result<()> {
        match key.code {
            // ALT-0..9 to switch tabs directly
            Char(c) if key.modifiers.contains(KeyModifiers::ALT) && c.is_ascii_digit() => {
                if let Some(digit) = c.to_digit(10) {
                    self.tab_manager.borrow_mut().switch(digit as usize);
                    self.status_bar
                        .borrow_mut()
                        .status(format!("Switched to tab {}", digit).as_str());
                }
            }
            // Show help
            KeyCode::F(1) => {
                // Add some test widgets
                let inner = Help::new();
                let w1 = Widget::new("help", 255, false, Rc::new(RefCell::new(inner)));
                self.widget_manager.create(w1);

                self.command_queue.push(Command::ShowWidget {
                    id: "help".into(),
                    focus: true,
                });
            }
            // Show tab list
            KeyCode::F(2) => {
                let inner = TabListWidget::new(self.tab_manager.clone());
                let widget = Widget::new("tab_list", 64, false, Rc::new(RefCell::new(inner)));
                self.widget_manager.create(widget);
                self.command_queue.push(Command::ShowWidget {
                    id: "tab_list".into(),
                    focus: true,
                });
            }
            // Show bookmark list
            KeyCode::F(8) => {
                let inner = BookmarkListWidget::new(self.bookmark_manager.clone());
                let widget = Widget::new("bookmark_list", 64, false, Rc::new(RefCell::new(inner)));
                self.widget_manager.create(widget);
                self.command_queue.push(Command::ShowWidget {
                    id: "bookmark_list".into(),
                    focus: true,
                });
            }
            // KeyCode::F(9) => self.menu_active = !self.menu_active,
            // Switch to previous tab
            KeyCode::Tab if key.modifiers.contains(KeyModifiers::SHIFT) => {
                let idx = self.tab_manager.borrow_mut().prev();
                self.status_bar
                    .borrow_mut()
                    .status(format!("Switched to tab {}", idx).as_str());
            }
            // switch to next tab
            KeyCode::Tab => {
                let idx = self.tab_manager.borrow_mut().next();
                self.status_bar
                    .borrow_mut()
                    .status(format!("Switched to tab {}", idx).as_str());
            }
            // Asks and opens URL in new tab
            Char('g') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                let name = "";
                let inner = InputWidget::new(
                    "Enter the URL to visit",
                    name,
                    80,
                    InputSubmitCommand::OpenTabWithUrl,
                );

                let widget = Widget::new("input", 64, false, Rc::new(RefCell::new(inner)));
                self.widget_manager.create(widget);
                self.command_queue.push(Command::ShowWidget {
                    id: "input".into(),
                    focus: true,
                });
            }
            // change the name of the current tab
            Char('r') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                let name = self.tab_manager.borrow().current().name.clone();

                let tab_idx = self.tab_manager.borrow().current;
                let inner = InputWidget::new(
                    "Enter name for this tab",
                    &name,
                    40,
                    InputSubmitCommand::RenameTab { tab_idx },
                );

                let widget = Widget::new("input", 64, false, Rc::new(RefCell::new(inner)));
                self.widget_manager.create(widget);
                self.command_queue.push(Command::ShowWidget {
                    id: "input".into(),
                    focus: true,
                });
            }
            // Close current tab
            Char('w') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                let idx = self.tab_manager.borrow().current;
                self.command_queue.push(Command::CloseTab { idx });
            }
            // Add new tab with blank page
            Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.command_queue.push(Command::NewTabUrl {
                    title: "New Tab".into(),
                    url: "gosub://blank".into(),
                });
            }
            // quit application
            Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.command_queue.push(Command::Quit);
            }
            _ => {}
        }
        Ok(())
    }

    pub(crate) fn process_commands(&mut self) {
        loop {
            match self.command_queue.pending() {
                None => break,
                Some(Command::Quit) => {
                    self.should_quit = true;
                    break;
                }
                Some(Command::ShowWidget { id, focus }) => {
                    self.widget_manager.show(&id, focus);
                }
                Some(Command::HideWidget { id }) => {
                    self.widget_manager.hide(&id);
                }
                Some(Command::ToggleWidget { id, focus }) => {
                    self.widget_manager.toggle(&id, focus);
                }
                Some(Command::FocusWidget { .. }) => {}
                Some(Command::UnfocusWidget { .. }) => {}
                Some(Command::DestroyWidget { id }) => {
                    self.widget_manager.destroy(&id);
                }
                Some(Command::InputSubmit { command, value }) => match command {
                    InputSubmitCommand::RenameTab { tab_idx } => {
                        self.command_queue.push(Command::RenameTab {
                            tab_idx,
                            name: value,
                        });
                    }
                    InputSubmitCommand::OpenTabWithUrl => {
                        self.command_queue.push(Command::NewTabUrl {
                            title: "New Tab".into(),
                            url: value,
                        });
                    }
                },
                Some(Command::RenameTab { tab_idx, name }) => {
                    self.tab_manager.borrow_mut().rename(tab_idx, &name);
                }
                Some(Command::NewTabUrl { title, url }) => {
                    let idx = self
                        .tab_manager
                        .borrow_mut()
                        .open(title.as_str(), url.as_str());
                    self.tab_manager.borrow_mut().switch(idx);
                    self.status_bar
                        .borrow_mut()
                        .status(format!("Opened new tab {}", idx).as_str());
                }
                Some(Command::CloseTab { idx }) => {
                    if self.tab_manager.borrow().len() == 1 {
                        self.status_bar.borrow_mut().status("Can't close last tab");
                        return;
                    }

                    self.tab_manager.borrow_mut().close(idx);
                    self.status_bar
                        .borrow_mut()
                        .status(format!("Closed tab {}", idx).as_str());

                    self.tab_manager.borrow_mut().close(idx);
                    self.status_bar
                        .borrow_mut()
                        .status(format!("Closed tab {}", idx).as_str());
                }
            }
        }
    }
}
