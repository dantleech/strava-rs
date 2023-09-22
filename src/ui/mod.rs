pub mod color;

use crate::{
    app::App,
    component::View,
};
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

    header(app).render(rows[0], f);
    view.draw(app, f, rows[1]);

    status_bar(app).render(rows[2], f);
}

fn header<'a>(_app: &'a mut App) -> Paragraph<'a> {
    let strava = ColorTheme::Orange.to_color();
    let text: Vec<Line> = vec![Line::from(vec![
        Span::styled("[n]", Style::default().fg(strava)),
        Span::raw("ext "),
        Span::styled("[p]", Style::default().fg(strava)),
        Span::raw("evious "),
        Span::styled("[u]", Style::default().fg(strava)),
        Span::raw("nit toggle "),
        Span::styled("[f]", Style::default().fg(strava)),
        Span::raw("ilter "),
        Span::styled("[s]", Style::default().fg(strava)),
        Span::raw("ort "),
        Span::styled("[S]", Style::default().fg(strava)),
        Span::raw("rank "),
        Span::styled("[o]", Style::default().fg(strava)),
        Span::raw("rder "),
        Span::styled("[r]", Style::default().fg(strava)),
        Span::raw("efresh "),
        Span::styled("[a]", Style::default().fg(strava)),
        Span::raw("nchor"),
        Span::styled("[+/-] ", Style::default().fg(strava)),
        Span::styled("[j]", Style::default().fg(strava)),
        Span::raw("down "),
        Span::styled("[k]", Style::default().fg(strava)),
        Span::raw("up "),
        Span::styled("[p]", Style::default().fg(strava)),
        Span::raw("evious "),
        Span::styled("[q]", Style::default().fg(strava)),
        Span::raw("uit"),
    ])];

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
