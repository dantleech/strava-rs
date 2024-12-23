use chrono::DateTime;
use tui::{
    layout::Constraint,
    prelude::Buffer,
    style::{Color, Style},
    symbols::Marker,
    text::{Line, Span},
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Widget},
};

use crate::{
    app::App,
    event::keymap::MappedKey,
    store::activity::{SortBy, SortOrder},
};

pub fn handle(_app: &mut App, _key: MappedKey) {}
pub fn draw(app: &mut App, f: &mut Buffer, area: tui::layout::Rect) {
    let activities = &app.activities().sort(&SortBy::Date, &SortOrder::Asc);
    let times: Vec<i64> = activities.timestamps();
    let paces: Vec<i64> = activities.meter_per_hours();
    let tmax = times.iter().max();
    let tmin = times.iter().min();
    let pmax = paces.iter().max();
    let pmin = paces.iter().min();
    if tmax.is_none() || tmin.is_none() {
        return;
    }
    let pdiff = pmax.unwrap() - pmin.unwrap();
    let tdiff = tmax.unwrap() - tmin.unwrap();
    if pmin.is_none() || pmax.is_none() {
        return;
    }
    let pmin = pmin.unwrap();
    let pmax = pmax.unwrap();
    let data: Vec<(f64, f64)> = activities
        .to_vec()
        .iter()
        .map(|a| {
            let ts = a.start_date.unwrap().and_utc().timestamp();
            (ts as f64, a.meters_per_hour())
        })
        .collect();
    let mut current = vec![];
    if let Some(selected) = app.activity_list.table_state().selected() {
        let activities = app.activities();
        if let Some(a) = activities.get(selected) {
            if let Some(a) = activities.find(a.id) {
                current.push((
                    a.start_date.unwrap().and_utc().timestamp() as f64,
                    *pmin as f64,
                ));
                current.push((
                    a.start_date.unwrap().and_utc().timestamp() as f64,
                    *pmax as f64,
                ));
            }
        }
    }
    let datasets = vec![
        Dataset::default()
            .name("Pace")
            .data(&data)
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::Blue)),
        Dataset::default()
            .name("Pace")
            .data(&data)
            .marker(Marker::Braille)
            .graph_type(GraphType::Scatter)
            .style(Style::default().fg(Color::Magenta)),
        Dataset::default()
            .data(&current)
            .name("Selected")
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::Green)),
    ];
    let yaxisstep = (pdiff as f64 / area.height as f64) as usize;
    let yaxis = (*pmin..*pmax).step_by(if yaxisstep > 0 { yaxisstep } else { 1 });
    let xaxisstep = (tdiff as f64 / 5.0) as usize;
    let xaxis = (*tmin.unwrap()..*tmax.unwrap()).step_by(if xaxisstep > 0 { xaxisstep } else { 1 });
    let chart = Chart::new(datasets)
        .hidden_legend_constraints((Constraint::Max(1), Constraint::Max(1)))
        .block(Block::default().borders(Borders::all()))
        .x_axis(
            Axis::default()
                .title(Span::styled("Date", Style::default().fg(Color::Red)))
                .style(Style::default().fg(Color::White))
                .bounds([*tmin.unwrap() as f64, *tmax.unwrap() as f64])
                .labels(xaxis.map(|p| {
                    Line::from(match DateTime::from_timestamp_millis(p * 1000) {
                        Some(t) => t.format("%Y-%m-%d").to_string(),
                        None => "n/a".to_string(),
                    })
                })),
        )
        .y_axis(
            Axis::default()
                .title(Span::styled("Pace", Style::default().fg(Color::Red)))
                .style(Style::default().fg(Color::White))
                .bounds([
                    *pmin as f64,
                    *pmax as f64 + (pdiff as f64 / activities.len() as f64),
                ])
                .labels(yaxis.map(|p| Span::from(app.unit_formatter.pace(3600, p as f64)))),
        );
    chart.render(area, f);
}
