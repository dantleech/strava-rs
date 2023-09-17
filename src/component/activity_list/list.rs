use crossterm::event::Event;
use tui::{
    backend::Backend,
    layout::Constraint,
    prelude::Buffer,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, StatefulWidget, Table, Widget},
    Frame,
};
use tui_input::backend::crossterm::EventHandler;

use crate::{
    app::App,
    component::{table_status_select_current, View},
    event::{
        input::InputEvent,
        keymap::{MappedKey, StravaEvent},
    },
    store::activity::{Activities, SortOrder},
    ui::{centered_rect_absolute, color::ColorTheme},
};

use super::{rank_dialog, sort_dialog};

pub fn activity_list_table<'a>(app: &App, activities: &'a Activities) -> Table<'a> {
    let mut rows = vec![];
    let header_names = [
        "Date",
        "",
        "Title",
        "Dst",
        "🕑 Time",
        "👣 Pace",
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
            Cell::from(app.unit_formatter.stopwatch_time(activity.moving_time)),
            Cell::from(
                app.unit_formatter
                    .pace(activity.moving_time, activity.distance),
            ),
            Cell::from(
                activity
                    .average_heartrate
                    .map_or_else(|| "n/a".to_string(), |v| format!("{:.2}", v)),
            ),
            Cell::from(app.unit_formatter.elevation(activity.total_elevation_gain)),
            Cell::from(format!("{}", activity.rank)),
        ]));
    }

    Table::new(rows)
        .header(
            Row::new(headers)
                .height(1)
                .bottom_margin(1)
                .style(Style::default()),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("")
        .widths(&[
            Constraint::Min(10),
            Constraint::Min(2),
            Constraint::Percentage(20),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
        ])
}
