pub struct Tab {
    pub name: String,
    pub url: String,
    pub content: String,
}

pub struct TabManager {
    pub tabs: Vec<Tab>,
    pub current: usize,
}

impl TabManager {
    pub fn new() -> Self {
        Self {
            tabs: vec![],
            current: 0,
        }
    }

    pub fn open(&mut self, name: &str, url: &str) -> usize {
        let tab = Tab {
            name: name.into(),
            url: url.into(),
            content: String::new(),
        };

        self.tabs.push(tab);

        self.tabs.len() - 1
    }

    pub fn switch(&mut self, idx: usize) -> usize {
        if idx < self.tabs.len() {
            self.current = idx;
        }

        self.current
    }

    pub fn next(&mut self) -> usize {
        self.current = (self.current + 1) % self.tabs.len();

        self.current
    }

    #[allow(dead_code)]
    pub fn prev(&mut self) -> usize {
        if self.current > 0 {
            self.current -= 1;
        } else {
            self.current = self.tabs.len() - 1;
        }

        self.current
    }

    pub fn close(&mut self, idx: usize) {
        if idx < self.tabs.len() {
            self.tabs.remove(idx);
            if self.current >= self.tabs.len() {
                self.current = self.tabs.len() - 1;
            }
        }
    }

    pub fn len(&self) -> usize {
        self.tabs.len()
    }
}
