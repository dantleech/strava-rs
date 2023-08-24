pub mod color;



use tui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    style::Style,
    text::{Span, Line, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::{
    app::{ActivePage, App},
    component::{activity_list, activity_view},
};

use self::color::ColorTheme;

pub fn draw<B: Backend>(app: &mut App, f: &mut Frame<B>) -> Result<(), anyhow::Error> {
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
        .split(f.size());

    f.render_widget(header(app), rows[0]);

    match app.active_page {
        ActivePage::ActivityList => {
            activity_list::draw(app, f, rows[1])?;
        }
        ActivePage::Activity => {
            activity_view::draw(app, f, rows[1])?;
        }
    }

    f.render_widget(status_bar(app), rows[2]);

    Ok(())
}

fn header<'a>(_app: &'a mut App) -> Paragraph<'a> {
    let strava = ColorTheme::Orange.to_color();
    let text: Vec<Line> = vec![Line::from(vec![
        Span::styled("[k]", Style::default().fg(strava)),
        Span::raw("up "),
        Span::styled("[j]", Style::default().fg(strava)),
        Span::raw("down "),
        Span::styled("[u]", Style::default().fg(strava)),
        Span::raw("nit toggle "),
        Span::styled("[f]", Style::default().fg(strava)),
        Span::raw("ilter "),
        Span::styled("[s]", Style::default().fg(strava)),
        Span::raw("ort "),
        Span::styled("[o]", Style::default().fg(strava)),
        Span::raw("rder "),
        Span::styled("[r]", Style::default().fg(strava)),
        Span::raw("efresh "),
        Span::styled("[a]", Style::default().fg(strava)),
        Span::raw("nchor "),
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
        status.push(format!("{} activities", app.filtered_activities().len()));
        status.push(format!(
            "sorted by {} {}",
            app.filters.sort_by, app.filters.sort_order
        ));
        status.push(format!("{} units", app.unit_formatter.system));
    }
    if let Some(anchored) = &app.activity_anchored {
        status.push(format!("anchored to \"{}\"", anchored.title));
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
