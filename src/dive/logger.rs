#[allow(dead_code)]
struct Logger {
    logs: Vec<String>,
    max_entries: usize,
}

impl Logger {
    #[allow(dead_code)]
    pub fn new(max_entries: usize) -> Self {
        Self {
            logs: vec![],
            max_entries,
        }
    }

    #[allow(dead_code)]
    pub fn log(&mut self, msg: &str) {
        self.logs.push(msg.into());
        if self.logs.len() > self.max_entries {
            self.logs.remove(0);
        }
    }
}