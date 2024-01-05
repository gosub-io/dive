use ratatui::Frame;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Clear, Padding, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap};
use crate::dive::app::App;
use crate::dive::widget_manager::Drawable;

const HELPTEXT: &'static str = r#"

#1Gosub Dive Help
#1===============
This is the help screen for Gosub Dive. It is a work in progress and displays the current key bindings. This browser is a proof-of-concept project and is not intended for production use.

 #2Function keys
 #2-------------
  #1F1#0      Display this help screen
  #1F2#0      Opens tab list
  #1F3#0
  #1F4#0
  #1F5#0
  #1F6#0      Opens log screen
  #1F7#0      Opens history menu
  #1F8#0      Opens bookmark menu
  #1F9#0      Opens top menu

 #2Navigation
 #2----------
  #1CTRL-N#0    Opens new tab with blank page
  #1CTRL-G#0    Asks for an URL to open
  #1CTRL-B#0    Browse back to previous page
  #1CTRL-R#0    Reload current page
  #1CTRL-W#0    Close current tab

 #2General commands
 #2----------------
  #1CTRL-Q#0    Quit Gosub Dive

 #2Tab management
 #2--------------
  #1ALT-0..9#0  Switch to tab 0..9
  #1CTRL-I#0    Rename tab
  #1TAB#0       Switch to next tab

"#;

fn generate_lines_from_helptext() -> Vec<Line<'static>> {
    // #0 is default style, #1 is yellow, etc
    let cols = vec![
        Style::default(),
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD),
        Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
        Style::default().fg(Color::LightBlue).add_modifier(Modifier::BOLD),
        Style::default().fg(Color::LightGreen).add_modifier(Modifier::BOLD),
    ];

    // This code basically iterates over the lines of the help text. Each line
    // is split into a vector of spans on the #N characters. If a #N is found,
    // the current span is saved, and a new styling for the next span is
    // created. This continues until we have reached the end of the line so
    // each line consists of 1 or more spans with the correct styling. It's
    // then rendered as a paragraph.

    let mut lines = Vec::new();
    let mut partial_line = Vec::new();

    let help_lines = HELPTEXT.split("\n").collect::<Vec<&str>>();
    for line in help_lines {
        let mut cs = Style::default();

        let mut start_idx = 0;
        let mut idx = 0;
        while idx < line.len() {
            let ch = line.chars().nth(idx).unwrap();
            match ch {
                '#' => {
                    if line.chars().nth(idx + 1).unwrap().is_ascii_digit() {
                        let line_part: String = line.chars().skip(start_idx).take(idx - start_idx).collect();
                        partial_line.push(Span::styled(line_part, cs));
                        start_idx = idx + 2;

                        let col_idx = line.chars().nth(idx + 1).unwrap().to_digit(10).unwrap();
                        cs = cols[col_idx as usize];
                        idx += 2;
                    } else {
                        // Seems like a regular #
                        idx += 1;
                    }
                }
                _ => idx += 1,
            }
        }

        let line_part: String = line.chars().skip(start_idx).take(idx - start_idx).collect();
        partial_line.push(Span::styled(line_part, cs));

        lines.push(Line::from(partial_line.clone()));
        partial_line = Vec::new();
    }

    lines
}

pub struct Help {
    pub vertical_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,
    pub vertical_scroll_max: usize,
    pub content: Vec<Line<'static>>,
}

impl Help {
    pub fn new() -> Self {
        // generate help text, based on #N coloring
        let help_lines = generate_lines_from_helptext();

        Self {
            vertical_scroll_state: ScrollbarState::default(),
            vertical_scroll: 0,
            vertical_scroll_max: help_lines.len(),
            content: help_lines,
        }
    }
}

impl Drawable for Help {
    fn render(&mut self, _app: &mut App, f: &mut Frame) {
        let size = f.size();
        let margins = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ])
            .split(size)
            ;

        let help_block_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ])
            .split(margins[1])[1]
            ;

        let help_block = Block::default()
            .title(" Help ")
            .borders(Borders::ALL)
            .padding(Padding::uniform(1))
            ;

        let help_paragraph = Paragraph::new(Text::from(self.content.clone()))
            .block(help_block)
            .wrap(Wrap { trim: false })
            .scroll((self.vertical_scroll as u16, 0))
            ;

        f.render_widget(Clear, help_block_area);
        f.render_widget(help_paragraph, help_block_area);

        f.render_stateful_widget(
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓")),
            help_block_area,
            &mut self.vertical_scroll_state,
        );
    }
}

// impl<T: 'static + Drawable> Help {
//     fn event_handler(&mut self, app: &mut App, key: KeyEvent) -> anyhow::Result<Option<KeyEvent>> {
//         match key.code {
//             KeyCode::Esc | KeyCode::F(1) => {
//                 app.widget_manager.find::<T>("help").unwrap().hide();
//                 // app.widget_manager.find("help").unwrap().unfocus();
//             }
//             KeyCode::Down => {
//                 self.vertical_scroll = self.vertical_scroll.saturating_add(1).clamp(0, self.vertical_scroll_max - 1);
//                 self.vertical_scroll_state = self.vertical_scroll_state.position(self.vertical_scroll);
//             },
//             KeyCode::Up => {
//                 self.vertical_scroll = self.vertical_scroll.saturating_sub(1);
//                 self.vertical_scroll_state = self.vertical_scroll_state.position(self.vertical_scroll);
//             },
//             Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => app.should_quit = true,
//             _ => {}
//         }
//
//         Ok(Some(key))
//     }
//
//     fn on_show(&mut self, app: &mut App) {
//         app.status_bar.status("Opened help screen");
//     }
//
//     fn on_hide(&mut self, app: &mut App) {
//         app.status_bar.status("Closed help screen");
//     }
// }

