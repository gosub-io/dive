use ratatui::prelude::*;
use ratatui::widgets::Paragraph;
use crate::dive::app::App;

pub fn menu_render(app: &mut App) -> Paragraph<'static> {
    let mut menu_tiles = vec![
        Span::styled(" Gosub Dive ", Style::default().fg(Color::White).bold()),
        Span::raw("|"),
        Span::raw(" File "),
        Span::raw("|"),
        Span::raw(" Edit "),
        Span::raw("|"),
        Span::raw(" View "),
        Span::raw("|"),
        Span::raw(" History "),
        Span::raw("|"),
        Span::raw(" Bookmarks "),
        Span::raw("|"),
        Span::raw(" Tools "),
        Span::raw("|"),
        Span::raw(" Help "),
    ];

    if app.menu_active {
        menu_tiles[2 + app.menu_item_active * 2] =
            Span::styled(
                menu_tiles[2 + app.menu_item_active * 2].content.clone(),
                Style::default().bg(Color::Green).fg(Color::White).add_modifier(Modifier::BOLD),
            )
        ;
    }

    Paragraph::new(Line::from(menu_tiles)).style(Style::default().bg(Color::Blue).add_modifier(Modifier::BOLD))
}