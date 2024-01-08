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

    pub fn current(&self) -> &Tab {
        self.tabs.get(self.current).expect("No current tab")
    }

    pub fn rename(&mut self, idx: usize, name: &str) {
        if idx < self.tabs.len() {
            self.tabs[idx].name = name.into();
        }
    }

    pub fn open(&mut self, name: &str, url: &str) -> usize {
        let tab = Tab {
            name: name.into(),
            url: url.into(),
            content: String::new(),
        };

        self.tabs.push(tab);
        log::debug!("Opening new tab: {}", url);

        self.tabs.len() - 1
    }

    pub fn switch(&mut self, idx: usize) -> usize {
        if idx < self.tabs.len() {
            self.current = idx;
        }

        log::trace!("Switching to tab: {}", self.current);
        self.current
    }

    pub fn next(&mut self) -> usize {
        self.current = (self.current + 1) % self.tabs.len();

        log::trace!("Switching to tab: {}", self.current);
        self.current
    }

    #[allow(dead_code)]
    pub fn prev(&mut self) -> usize {
        if self.current > 0 {
            self.current -= 1;
        } else {
            self.current = self.tabs.len() - 1;
        }

        log::trace!("Switching to tab: {}", self.current);
        self.current
    }

    pub fn close(&mut self, idx: usize) {
        if idx < self.tabs.len() {
            self.tabs.remove(idx);
            if self.current >= self.tabs.len() {
                self.current = self.tabs.len() - 1;
            }

            log::trace!("Closed tab {}", idx);
        }
    }

    pub fn len(&self) -> usize {
        self.tabs.len()
    }
}
