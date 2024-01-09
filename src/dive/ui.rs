use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::Frame;
use std::rc::Rc;

pub fn centered_rect_fixed(width: u16, height: u16, r: Rect) -> Rect {
    let position = ((r.width - width) / 2, (r.height - height) / 2);

    Rect::new(position.0, position.1, width, height)
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

/// Returns the layout chunks for the main screen
pub fn get_layout_chunks(f: &mut Frame) -> Rc<[Rect]> {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        // .margin(1)
        .constraints(
            [
                Constraint::Length(1), // menu bar
                Constraint::Min(0),    // content
                Constraint::Length(1), // status bar
            ]
            .as_ref(),
        )
        .split(size);

    chunks
}
