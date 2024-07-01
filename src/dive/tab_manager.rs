use anyhow::Error;
use crate::dive::widgets::status_bar::TabInfo;
use ureq;
use url::Url;

pub struct Tab {
    pub name: String,
    pub url: String,
    pub content: String,
    pub secure: bool,
}

impl Tab {
    pub fn info(&self) -> TabInfo {
        TabInfo {
            name: self.name.clone(),
            url: self.url.clone(),
            secure: self.secure,
        }
    }
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
        let mut tab = Tab {
            name: name.into(),
            url: url.into(),
            content: String::new(),
            secure: false,
        };

        tab.secure = url.starts_with("https://");
        tab.content = load_content(url).expect("Failed to load content").clone();

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

fn load_content(url: &str) -> Result<String, anyhow::Error> {
    let parts = match Url::parse(url) {
        Ok(parts) => parts,
        Err(e) => return Err(anyhow::Error::msg(format!("Invalid URL: {}", e))),
    };

    if parts.scheme() == "file" {
        return Ok("File content is not yet supported".into());
    }
    if parts.scheme() == "data" {
        return Ok("Data content is not yet supported".into());
    }
    if parts.scheme() == "gosub" {
        return process_gosub_protocol(parts);
    }

    if parts.scheme() == "" || parts.scheme() == "https" {
        let content = ureq::get(url).call()?.into_string()?;
        return Ok(content);
    }
    if parts.scheme() == "" || parts.scheme() == "http" {
        log::warn!("Opening insecure connection to {}", url);
        let content = ureq::get(url).call()?.into_string()?;
        return Ok(content);
    }

    // Always assume no protocol defaults to HTTPS://

    Ok("Unknown protocol".into())
}

fn process_gosub_protocol(url: Url) -> Result<String, Error> {
    match url.host_str() {
        Some("blank") => return Ok("This page is left intentionally blank".into()),
        Some("help") => return Ok(gosub_help()),
        Some("credits") => return Ok("Here be credits for the gosub engine".into()),
        Some("settings") => return Ok("Here you can tinker with all kinds of dive and gosub settings".into()),
        _ => return Ok("Unknown gosub protocol".into()),
    }
}

fn gosub_help() -> String {
    return r#"<h1>gosub://help</h1>

    <p>This is the help page for the gosub engine</p>

    <p>The following special gosub pages are supported:</p>

    <table>
      <tr><td><a target="_blank"href="gosub://blank">gosub://blank</td><td>Opens a blank page</td></tr>
      <tr><td><a target="_blank"href="gosub://help">gosub://help</td><td>Displays this help page</td></tr>
      <tr><td><a target="_blank" href="gosub://credits">gosub://credits</td><td>Displays credits of the Dive Browser and the Gosub Engine</td></tr>
      <tr><td><a target="_blank" href="gosub://settings">gosub://settings</td><td>Displays the settings page</td></tr>
    </table>
    "#.into();
}
