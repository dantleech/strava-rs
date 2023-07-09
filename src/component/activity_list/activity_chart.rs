use chrono::{serde::ts_seconds_option, NaiveDate, NaiveDateTime};
use tui::{
    backend::Backend,
    layout::Constraint,
    style::{Color, Modifier, Style},
    symbols::Marker,
    text::Span,
    widgets::{Axis, Block, Borders, Cell, Chart, Clear, Dataset, GraphType, Row, Table},
    Frame,
};

use crate::{
    app::{App, SortOrder},
    component::table_status_select_current,
    event::{
        keymap::{MappedKey, StravaEvent},
        util::{table_state_next, table_state_prev},
    },
    store::activity::Activity,
    ui::{centered_rect_absolute, color::ColorTheme, key_event_to_input},
};

use super::sort_dialog;

pub fn handle(app: &mut App, key: MappedKey) {}
pub fn draw<B: Backend>(
    app: &mut App,
    f: &mut Frame<B>,
    area: tui::layout::Rect,
) -> Result<(), anyhow::Error> {
    let activities = &app.unsorted_filtered_activities();
    let times: Vec<i64> = activities
        .iter()
        .map(|a| {
            return a.start_date.unwrap().timestamp();
        })
        .collect();
    let paces: Vec<i64> = activities
        .iter()
        .map(|a| {
            return a.meters_per_hour() as i64;
        })
        .collect();
    let tmax = times.iter().max();
    let tmin = times.iter().min();
    let pmax = paces.iter().max();
    let pmin = paces.iter().min();
    if None == tmax || None == tmin {
        return Ok(());
    }
    let pdiff = pmax.unwrap() - pmin.unwrap();
    let tdiff = tmax.unwrap() - tmin.unwrap();
    if None == pmin || None == pmax {
        return Ok(());
    }
    let pmin = pmin.unwrap();
    let pmax = pmax.unwrap();
    let data: Vec<(f64, f64)> = activities
        .iter()
        .map(|a| {
            let ts = a.start_date.unwrap().timestamp();
            return (ts as f64, a.meters_per_hour() as f64);
        })
        .collect();
    let mut current = vec![];
    if let Some(selected) = app.activity_list_table_state.selected() {
        let activities = app.filtered_activities();
        if let Some(a) = activities.get(selected) {
            match app.activities.iter().find(|unsorted|unsorted.id == a.id) {
                Some(a) => {
                    current.push((a.start_date.unwrap().timestamp() as f64, pmin.clone() as f64));
                    current.push((a.start_date.unwrap().timestamp() as f64, pmax.clone() as f64));
                },
                None => (),
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
        Dataset::default().data(&current)
            .name("Selected")
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::Green)),
    ];
    let yaxis =
        (*pmin..*pmax).step_by((pdiff as f64 / area.height as f64) as usize);
    let xaxis = (*tmin.unwrap()..*tmax.unwrap()).step_by((tdiff as f64 / 5.0) as usize);
    let chart = Chart::new(datasets)
        .block(Block::default().borders(Borders::all()))
        .x_axis(
            Axis::default()
                .title(Span::styled("Date", Style::default().fg(Color::Red)))
                .style(Style::default().fg(Color::White))
                .bounds([*tmin.unwrap() as f64, *tmax.unwrap() as f64])
                .labels(
                    xaxis
                        .map(|p| {
                            Span::from(match NaiveDateTime::from_timestamp_millis(p * 1000) {
                                Some(t) => t.format("%Y-%m-%d").to_string(),
                                None => "n/a".to_string(),
                            })
                        })
                        .collect(),
                ),
        )
        .y_axis(
            Axis::default()
                .title(Span::styled("Pace", Style::default().fg(Color::Red)))
                .style(Style::default().fg(Color::White))
                .bounds([*pmin as f64, *pmax as f64 + (pdiff as f64 / activities.len() as f64)])
                .labels(
                    yaxis
                        .map(|p| Span::from(app.unit_formatter.pace(3600, p as f32)))
                        .collect(),
                ),
        );
    f.render_widget(chart, area);

    Ok(())
}
