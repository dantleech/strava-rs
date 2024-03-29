
use strum::IntoEnumIterator;
use tui::{
    style::{Color, Style},
    text::{Span, Line},
    widgets::{Block, Borders, Clear, Paragraph, Widget}, prelude::Buffer,
};

use crate::{
    app::App,
    event::{keymap::{MappedKey, StravaEvent}, input::InputEvent},
    ui::{centered_rect_absolute, color::ColorTheme}, store::activity::SortBy,
};

pub fn handle(app: &mut App, key: MappedKey) {
    let matched = match key.strava_event {
        StravaEvent::Enter => {
            app.activity_list.sort_dialog = false;
            true
        }
        StravaEvent::Escape => {
            app.activity_list.sort_dialog = false;
            true
        }
        _ => false,
    };

    if matched {
        return;
    }

    if let Some(sort) = SortBy::from_key(key.key_event.code) {
        app.filters.sort_by = sort;
        app.activity_list.sort_dialog = false;
        app.send(InputEvent::Reload);
    }
}

pub fn draw(
    app: &mut App,
    f: &mut Buffer,
    area: tui::layout::Rect,
) {
    let rect = centered_rect_absolute(64, 3, area);
    Clear.render(rect, f);
    sort_option_paragraph(app, "Sort".to_string()).render(rect, f);
}
pub fn sort_option_paragraph<'a>(_app: &'a mut App, title: String) -> Paragraph<'a> {
    let strava = ColorTheme::Orange.to_color();
    let mut sorts = vec![];

    for sort_by in SortBy::iter() {
        sorts.push(Span::styled(
            format!("[{}]", sort_by.to_key()),
            Style::default().fg(strava),
        ));
        sorts.push(Span::styled(
            format!("{} ", sort_by.to_label()),
            Style::default().fg(Color::White),
        ));
    }

    sorts.push(Span::styled(
        "<Enter> ".to_string(),
        Style::default().fg(strava),
    ));
    sorts.push(Span::styled(
        "cancel ".to_string(),
        Style::default().fg(Color::White),
    ));

    let text: Vec<Line> = vec![Line::from(sorts)];

    Paragraph::new(text).block(
        Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ColorTheme::Dialog.to_color()))
            .style(Style::default()),
    )
}
