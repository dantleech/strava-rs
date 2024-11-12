pub mod color;

use crate::{app::App, component::View, event::keymap::StravaEvent};
use crossterm::event::KeyCode;
use tui::{
    layout::{Constraint, Layout, Rect},
    prelude::Buffer,
    style::Style,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Widget},
};

use self::color::ColorTheme;

pub fn draw(app: &mut App, f: &mut Buffer, area: Rect, view: &mut dyn View) {
    let rows = Layout::default()
        .margin(0)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(4),
                Constraint::Length(if app.error_message.is_none() { 1 } else { 2 }),
            ]
            .as_ref(),
        )
        .split(area);

    header(app, view.mapped_events(app)).render(rows[0], f);

    view.draw(app, f, rows[1]);

    status_bar(app).render(rows[2], f);
}

fn header<'a>(app: &'a mut App, mapped_events: Vec<StravaEvent>) -> Paragraph<'a> {
    let strava = ColorTheme::Orange.to_color();
    let mut hints: Vec<Span> = vec![];
    for event in mapped_events {
        match app.key_map.key(&event) {
            Some(k) => if let KeyCode::Char(c) = k {
                hints.push(Span::styled(
                    format!("[{}]", c),
                    Style::default().fg(strava),
                ));
                hints.push(Span::raw(format!("{} ", StravaEvent::describe(&event))));
            },
            None => continue,
        }
    }

    let text: Vec<Line> = vec![Line::from(hints)];
    Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default()),
    )
}

fn status_bar<'a>(app: &'a mut App) -> Paragraph<'a> {
    let mut status: Vec<String> = Vec::new();
    if let Some(message) = &app.info_message {
        status.push(message.to_string());
    } else {
        if app.filters.filter != *"" {
            status.push(format!("filtered by \"{}\"", app.filters.filter))
        }
        status.push(format!("{} activities", app.activities().len()));
        status.push(format!(
            "sorted by {} {}",
            app.filters.sort_by, app.filters.sort_order
        ));
        status.push(format!(
            "ranked by {} {}",
            app.ranking.rank_by, app.ranking.rank_order
        ));
        status.push(format!("{} units", app.unit_formatter.system));
        if let Some(anchored) = &app.activity_anchored {
            status.push(format!(
                "anchored to \"{}\" ± {:.3}",
                anchored.title, app.filters.anchor_tolerance
            ));
        }
    }

    if let Some(message) = &app.error_message {
        status.push(format!("\n{}", message));
    }

    Paragraph::new(Text::from(status.join(", ")))
}

// borrowed from https://github.com/extrawurst/gitui
pub fn centered_rect_absolute(width: u16, height: u16, r: Rect) -> Rect {
    Rect::new(
        (r.width.saturating_sub(width)) / 2,
        (r.height.saturating_sub(height)) / 2,
        width.min(r.width),
        height.min(r.height),
    )
}
