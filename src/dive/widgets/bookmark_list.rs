use crate::dive::bookmark_manager::{BookmarkManager, Folder};
use crate::dive::command_queue::{Command, CommandQueue};
use crate::dive::ui::centered_rect;
use crate::dive::widget_manager::Drawable;
use crossterm::event::KeyCode::Char;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Clear, Row, Table, TableState};
use ratatui::Frame;
use std::cell::RefCell;
use std::rc::Rc;
use tui_tree_widget::{Tree, TreeItem, TreeState};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Selection {
    // Bookmark folder tree is currently in focus
    Tree,
    // Bookmark list is currently in focus
    Table,
}

pub struct BookmarkListWidget {
    bookmark_manager: Rc<RefCell<BookmarkManager>>,
    tree_state: TreeState<Uuid>,
    table_state: TableState,
    items: Vec<TreeItem<'static, Uuid>>,
    selection: Selection,
}

impl BookmarkListWidget {
    pub fn new(bookmark_manager: Rc<RefCell<BookmarkManager>>) -> Self {
        let bm = bookmark_manager.clone();

        Self {
            bookmark_manager: bm.clone(),
            tree_state: TreeState::default(),
            items: vec![],
            selection: Selection::Tree,
            table_state: TableState::default(),
        }
    }
}

impl BookmarkListWidget {
    /// Returns the folder (cloned) that has been selected in the tree
    fn get_selected_folder(&self) -> Folder {
        self.bookmark_manager
            .borrow()
            .find_folder(*self.tree_state.selected().last().unwrap())
            .unwrap()
    }
}

impl Drawable for BookmarkListWidget {
    /// Called when the widget is shown. It will create the tree from the current bookmark list
    fn on_show(&mut self) {
        let root = self.bookmark_manager.borrow().root();
        self.items =
            vec![TreeItem::new(root.id, root.name.clone(), generate_tree(&root)).expect("reason")];

        self.tree_state = TreeState::default();
        self.tree_state.select_first(self.items.as_slice());

        // Opening up all first level folders
        self.tree_state.open(vec![root.id]);
        for dir in self
            .bookmark_manager
            .borrow()
            .find_folder(root.id)
            .unwrap()
            .subfolders
            .iter()
        {
            self.tree_state.open(vec![dir.id]);
        }
    }

    fn on_hide(&mut self) {
        self.items = vec![];
    }

    fn render(&mut self, f: &mut Frame) {
        let area = centered_rect(100, 75, f.size());
        f.render_widget(Clear, area);

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(area);

        let mut tree = Tree::new(self.items.clone())
            .expect("reason")
            .block(Block::default().borders(Borders::LEFT | Borders::BOTTOM | Borders::TOP));

        // Read bookmarks from the selected folder
        let mut rows = vec![];
        for bookmark in self.get_selected_folder().bookmarks {
            rows.push(Row::new(vec![bookmark.title, "today".into()]));
        }

        let widths = [Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)];
        let mut table = Table::new(rows, widths)
            .block(Block::default().borders(Borders::ALL))
            .header(
                Row::new(vec!["Name", "Last visited"])
                    .style(Style::default().fg(Color::Yellow))
                    .bottom_margin(1),
            );

        // Highlight the selected widget (tree or table)
        if self.selection == Selection::Tree {
            tree = tree.highlight_style(
                Style::new()
                    .fg(Color::Yellow)
                    .bg(Color::Red)
                    .add_modifier(Modifier::BOLD),
            );
        } else {
            tree = tree.highlight_style(Style::new().fg(Color::Black).bg(Color::Gray));

            table = table.highlight_style(
                Style::new()
                    .fg(Color::Yellow)
                    .bg(Color::Red)
                    .add_modifier(Modifier::BOLD),
            );
        }

        f.render_stateful_widget(tree, layout[0], &mut self.tree_state);
        f.render_stateful_widget(table, layout[1], &mut self.table_state);
    }

    fn event_handler(
        &mut self,
        queue: &mut CommandQueue,
        key: KeyEvent,
    ) -> anyhow::Result<Option<KeyEvent>> {
        match key.code {
            KeyCode::Esc => {
                queue.push(Command::DestroyWidget {
                    id: "bookmark_list".into(),
                });
            }
            KeyCode::Right => {
                if self.selection == Selection::Tree {
                    self.tree_state.key_right();
                }
            }
            KeyCode::Down => {
                if self.selection == Selection::Tree {
                    self.tree_state.key_down(self.items.as_slice());
                } else {
                    let sel = self.table_state.selected().unwrap_or(0);
                    if sel < self.get_selected_folder().bookmarks.len() - 1 {
                        self.table_state.select(Some(sel + 1));
                    }
                }
            }
            KeyCode::Up => {
                if self.selection == Selection::Tree {
                    self.tree_state.key_up(self.items.as_slice());
                } else {
                    let sel = self.table_state.selected().unwrap_or(0);
                    if sel > 0 {
                        self.table_state.select(Some(sel - 1));
                    }
                }
            }
            KeyCode::Left => {
                if self.selection == Selection::Tree {
                    self.tree_state.key_left();
                }
            }
            KeyCode::Enter => {
                // let sel = self.state.selected().unwrap_or(0);
                // self.bookmark_manager.borrow_mut().switch(sel);
                // queue.push(Command::DestroyWidget {
                //     id: "bookmark_list".into(),
                // });
            }
            KeyCode::Tab => {
                // Switch beteen table and tree view
                if self.selection == Selection::Tree {
                    self.selection = Selection::Table;
                    self.table_state.select(Some(0));
                } else {
                    self.selection = Selection::Tree;
                    self.table_state.select(Some(0));
                }
            }
            Char(' ') => {
                if self.selection == Selection::Tree {
                    self.tree_state.toggle_selected();
                }
            }
            Char('/') => {
                // search for tags
            }
            _ => {}
        }

        Ok(Some(key))
    }
}

// Generate a tree for the tree-widget. This is a recursive function.
fn generate_tree(folder: &Folder) -> Vec<TreeItem<'static, Uuid>> {
    let mut items = vec![];

    for subfolder in folder.subfolders.iter() {
        let subitems = generate_tree(subfolder);
        let item = TreeItem::new(subfolder.id, subfolder.name.clone(), subitems).expect("reason");
        items.push(item);
    }

    items
}
