use chrono::{DateTime, Local};
use log::Level;
use std::sync::{Arc, Mutex};

pub struct LogRecord {
    pub msg: String,
    pub level: Level,
    pub timestamp: DateTime<Local>,
}

#[derive(Default)]
pub struct LogPool {
    logs: Vec<LogRecord>,
    max_entries: usize,
}

impl LogPool {
    pub fn new(max_entries: usize) -> Self {
        Self {
            logs: vec![],
            max_entries,
        }
    }

    pub fn log(&mut self, record: LogRecord) {
        self.logs.push(record);
        if self.logs.len() > self.max_entries {
            self.logs.remove(0);
        }
    }

    pub fn logs(&self) -> &[LogRecord] {
        &self.logs
    }
}

#[derive(Default)]
pub struct GosubLogger {
    pool: Arc<Mutex<LogPool>>,
}

impl GosubLogger {
    pub fn new(pool: Arc<Mutex<LogPool>>) -> Self {
        Self { pool }
    }
}

impl log::Log for GosubLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            self.pool.lock().unwrap().log(LogRecord {
                msg: format!("{}", record.args()),
                level: record.level(),
                timestamp: Local::now(),
            });
        }
    }

    fn flush(&self) {}
}
