use crate::dive::widgets::input::InputSubmitCommand;
use std::collections::VecDeque;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Command {
    HideWidget {
        id: String,
    },
    ShowWidget {
        id: String,
        focus: bool,
    },
    ToggleWidget {
        id: String,
        focus: bool,
    },
    FocusWidget {
        id: String,
    },
    UnfocusWidget {
        id: String,
    },
    DestroyWidget {
        id: String,
    },
    Quit,
    InputSubmit {
        command: InputSubmitCommand,
        value: String,
    },
    RenameTab {
        tab_idx: usize,
        name: String,
    },
}

pub struct CommandQueue {
    commands: VecDeque<Command>,
}

impl CommandQueue {
    pub fn new() -> Self {
        // log::trace!("Creating new command queue");

        Self {
            commands: VecDeque::new(),
        }
    }

    pub fn push(&mut self, command: Command) {
        // log::trace!("Adding command to queue: {:?}", command);
        self.commands.push_back(command);
    }

    #[allow(dead_code)]
    pub fn pending(&mut self) -> Option<Command> {
        self.commands.pop_front()
    }
}
