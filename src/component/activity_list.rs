use tui::{
    backend::Backend,
    layout::Constraint,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Cell, Row, Table, TableState},
    Frame,
};

use crate::{
    app::App,
    event::{
        keymap::{MappedKey, StravaEvent},
        util::{table_state_next, table_state_prev},
    },
    store::activity::{Activity, ActivityStore},
};

pub fn handle(app: &mut App, key: MappedKey) {
    match key.strava_event {
        StravaEvent::ToggleUnitSystem => {
            app.unit_formatter = app.unit_formatter.toggle();
        }
        StravaEvent::Down => table_state_next(app.activity_list_table_state, app.activities.len()),
        StravaEvent::Up => table_state_prev(app.activity_list_table_state, app.activities.len()),
        _ => (),
    }
}

pub fn draw<B: Backend>(
    app: &App,
    f: &mut Frame<B>,
    area: tui::layout::Rect,
) -> Result<(), anyhow::Error> {
    let mut rows = vec![];
    let header_names = ["Date", "", "Title", "Dst", "🕑", "🚤", "👣", "💓", "🌄"];
    let headers = header_names
        .iter()
        .map(|header| Cell::from(Span::styled(*header, Style::default().fg(Color::DarkGray))));

    let activities = app.activities;
    for activity in activities {
        rows.push(Row::new([
            Cell::from(match activity.start_date {
                Some(x) => x.format("%Y-%m-%d").to_string(),
                None => "".to_string(),
            }),
            Cell::from(match activity.activity_type.as_str() {
                "Ride" => "🚴".to_string(),
                "Run" => "🏃".to_string(),
                "TrailRun" => "🏃🌲".to_string(),
                "Walk" => "🥾".to_string(),
                "WeightTraining" => "󱅝".to_string(),
                _ => activity.activity_type.clone(),
            }),
            Cell::from(activity.title.clone()),
            Cell::from(app.unit_formatter.distance(activity.distance)),
            Cell::from(app.unit_formatter.stopwatch_time(activity.moving_time)),
            Cell::from(
                app.unit_formatter
                    .speed(activity.distance, activity.moving_time),
            ),
            Cell::from(
                app.unit_formatter
                    .pace(activity.moving_time, activity.distance),
            ),
            Cell::from(
                activity
                    .average_heartrate
                    .map_or_else(|| "n/a".to_string(), |v| v.to_string()),
            ),
            Cell::from(app.unit_formatter.elevation(activity.total_elevation_gain)),
        ]));
    }

    let table = Table::new(rows)
        .header(
            Row::new(headers)
                .height(1)
                .bottom_margin(1)
                .style(Style::default()),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Min(10),
            Constraint::Min(4),
            Constraint::Percentage(20),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
        ]);

    f.render_stateful_widget(table, area, &mut app.activity_list_table_state);
    Ok(())
}
