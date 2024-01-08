use serde::{Deserialize, Serialize};
use std::fs;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct Bookmark {
    /// Title of the bookmark, could be the title of the website
    pub title: String,
    /// Url
    pub url: String,
    /// Additional tags to categorize the bookmark
    pub tags: Vec<String>,
    /// Any keywords that allows you to quickly search for the bookmark in the URL bar
    pub keywords: Vec<String>,
    /// Last time the bookmark was visited
    #[serde(default)]
    pub last_visited: u64,
    /// Id of the bookmark (internal only)
    #[serde(default = "uuid_default")]
    pub id: uuid::Uuid,
}

fn uuid_default() -> uuid::Uuid {
    uuid::Uuid::new_v4()
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Folder {
    /// Id of the folder (internal only)
    #[serde(default = "uuid_default")]
    pub id: uuid::Uuid,
    /// Name of the folder
    pub name: String,
    /// Any subfolders
    pub subfolders: Vec<Folder>,
    /// Any bookmarks found in this folder
    pub bookmarks: Vec<Bookmark>,
}

#[derive(Clone)]
pub struct BookmarkManager {
    pub root: Folder,
}

impl BookmarkManager {
    pub fn empty() -> Self {
        Self {
            root: Folder {
                id: Uuid::new_v4(),
                name: "root".into(),
                subfolders: vec![],
                bookmarks: vec![],
            },
        }
    }

    pub fn new_from_file(path: &str) -> Self {
        let root = read_bookmarks_config(path);
        match root {
            Ok(root) => Self { root },
            Err(_) => Self::empty(),
        }
    }

    pub fn root(&self) -> Folder {
        self.root.clone()
    }

    pub fn find_folder(&self, id: Uuid) -> Option<Folder> {
        find_folder_recursive(&self.root, id)
    }

    #[allow(dead_code)]
    pub fn find_bookmark(&self, id: Uuid) -> Option<Bookmark> {
        find_bookmark_recursive(&self.root, id)
    }
}

fn read_bookmarks_config(file_path: &str) -> Result<Folder, serde_json::Error> {
    let file_content = fs::read_to_string(file_path).expect("Unable to read the file");

    serde_json::from_str(&file_content)
}

fn find_folder_recursive(folder: &Folder, id: Uuid) -> Option<Folder> {
    if folder.id == id {
        return Some(folder.clone());
    }

    for subfolder in folder.subfolders.iter() {
        let found = find_folder_recursive(subfolder, id);
        if found.is_some() {
            return found;
        }
    }

    None
}

#[allow(dead_code)]
fn find_bookmark_recursive(folder: &Folder, id: Uuid) -> Option<Bookmark> {
    for bookmark in folder.bookmarks.iter() {
        if bookmark.id == id {
            return Some(bookmark.clone());
        }
    }

    for subfolder in folder.subfolders.iter() {
        let found = find_bookmark_recursive(subfolder, id);
        if found.is_some() {
            return found;
        }
    }

    None
}
