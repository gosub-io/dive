use std::rc::Rc;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};

struct Logger {
    logs: Vec<String>,
    max_entries: usize,
}

impl Logger {
    pub fn new(max_entries: usize) -> Self {
        Self {
            logs: vec![],
            max_entries,
        }
    }

    pub fn log(&mut self, msg: &str) {
        self.logs.push(msg.into());
        if self.logs.len() > self.max_entries {
            self.logs.remove(0);
        }
    }
}