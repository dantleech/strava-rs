use tui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Cell, Row, Table},
};

use crate::{app::App, store::activity::Activities};

pub fn activity_list_table<'a>(app: &App, activities: &'a Activities) -> Table<'a> {
    let mut rows = vec![];
    let time_header = match app.activity_list.use_moving_time {
        true => "🕑 Moving",
        false => "🕑 Elapsed",
    };
    let header_names = [
        "Date",
        "",
        "Title",
        "Dst",
        time_header,
        "👣 Pace",
        "󰓅  Speed",
        "💓 Avg. Heart",
        "🌄 Elevation",
        "🪜 Rank",
    ];
    let headers = header_names
        .iter()
        .map(|header| Cell::from(Span::styled(*header, Style::default().fg(Color::DarkGray))));

    for activity in activities.to_vec() {
        rows.push(Row::new([
            Cell::from(match activity.start_date {
                Some(x) => x.format("%Y-%m-%d").to_string(),
                None => "".to_string(),
            }),
            Cell::from(activity.activity_type_icon()),
            Cell::from(activity.title.clone()),
            Cell::from(app.unit_formatter.distance(activity.distance)),
            Cell::from(app.unit_formatter.stopwatch_time(
                match app.activity_list.use_moving_time {
                    true => activity.moving_time,
                    false => activity.elapsed_time,
                },
            )),
            Cell::from(match app.activity_list.use_moving_time {
                true => app
                    .unit_formatter
                    .pace(activity.moving_time, activity.distance),
                false => app
                    .unit_formatter
                    .pace(activity.elapsed_time, activity.distance),
            }),
            Cell::from(app.unit_formatter.speed(activity.meters_per_hour())),
            Cell::from(
                activity
                    .average_heartrate
                    .map_or_else(|| "n/a".to_string(), |v| format!("{:.2}", v)),
            ),
            Cell::from(app.unit_formatter.elevation(activity.total_elevation_gain)),
            Cell::from(format!("{}", activity.rank)),
        ]));
    }

    Table::new(
        rows,
        &[
            Constraint::Length(10),
            Constraint::Length(2),
            Constraint::Percentage(20),
            Constraint::Min(8),
            Constraint::Min(8),
            Constraint::Min(8),
            Constraint::Min(8),
            Constraint::Min(8),
            Constraint::Min(8),
            Constraint::Min(8),
        ],
    )
    .header(
        Row::new(headers)
            .height(1)
            .bottom_margin(1)
            .style(Style::default()),
    )
    .highlight_style(Style::default().add_modifier(Modifier::BOLD))
    .highlight_symbol("")
}
