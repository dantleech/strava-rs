use crossterm::event::KeyCode;
use strum::IntoEnumIterator;
use tui::{
    backend::Backend,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::{
    app::{App, SortBy},
    event::keymap::{MappedKey, StravaEvent},
    ui::{centered_rect_absolute, color::ColorTheme},
};

impl SortBy {
    pub fn to_key(&self) -> char {
        match *self {
            SortBy::Date => 'd',
            SortBy::Pace => 'p',
            SortBy::HeartRate => 'h',
            SortBy::Distance => 'D',
        }
    }

    pub fn to_label(&self) -> &str {
        match *self {
            SortBy::Date => "date",
            SortBy::Pace => "pace",
            SortBy::HeartRate => "heartrate",
            SortBy::Distance => "distance",
        }
    }

    pub fn from_key(key: KeyCode) -> Option<SortBy> {
        match key {
            KeyCode::Char('d') => Some(SortBy::Date),
            KeyCode::Char('p') => Some(SortBy::Pace),
            KeyCode::Char('h') => Some(SortBy::HeartRate),
            KeyCode::Char('D') => Some(SortBy::Distance),
            _ => None,
        }
    }
}

pub fn handle(app: &mut App, key: MappedKey) {
    let matched = match key.strava_event {
        StravaEvent::Enter => {
            app.activity_list_sort_dialog = false;
            true
        }
        _ => false,
    };

    if matched {
        return;
    }

    if let Some(sort) = SortBy::from_key(key.key_event.code) {
        app.activity_list_sort_by = sort;
        app.activity_list_sort_dialog = false;
    }
}

pub fn draw<B: Backend>(
    app: &mut App,
    f: &mut Frame<B>,
    area: tui::layout::Rect,
) -> Result<(), anyhow::Error> {
    let rect = centered_rect_absolute(64, 3, area);
    f.render_widget(Clear, rect);
    let block = Block::default()
        .title("Sort".to_string())
        .borders(Borders::ALL)
        .style(Style::default().fg(ColorTheme::Dialog.to_color()));

    f.render_widget(block, rect);
    f.render_widget(sort_option_paragraph(app), rect);

    Ok(())
}
fn sort_option_paragraph<'a>(_app: &'a mut App) -> Paragraph<'a> {
    let strava = ColorTheme::Orange.to_color();
    let mut sorts = vec![];

    for sort_by in SortBy::iter() {
        sorts.push(Span::styled(
            format!("[{}]", sort_by.to_key()),
            Style::default().fg(strava),
        ));
        sorts.push(Span::styled(
            format!("{} ", sort_by.to_label().to_string()),
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

    let text: Vec<Spans> = vec![Spans::from(sorts)];

    Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default()),
    )
}
