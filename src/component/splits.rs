
use std::f64::MAX;


use tui::{
    layout::{Constraint},
    style::{Style, Styled, Modifier},
    widgets::{Row, Cell, Table, StatefulWidget}, prelude::Buffer,
};

use crate::{app::App, store::activity::{ActivitySplit, SportType}, ui::color::{gradient, Rgb}};

pub fn draw(
    app: &mut App,
    f: &mut Buffer,
    area: tui::layout::Rect,
) {
    if app.activity.is_none() {
        return;
    }
    let activity = app.activity.as_ref().unwrap();
    // TODO: cant use async DB access here
    let splits: &Vec<ActivitySplit> = activity.splits.as_ref();
    let mut constraints = vec![];
    constraints.push(Constraint::Max(1));

    let mut max = 0.0;
    let mut min = MAX;
    for split in splits.iter() {
        constraints.push(Constraint::Max(1));
        if split.seconds_per_meter() > max {
            max = split.seconds_per_meter()
        }
        if split.seconds_per_meter() < min {
            min = split.seconds_per_meter()
        }
    }
    constraints.push(Constraint::Max(0));

    let mut rows = vec![];
    let speed_header = match activity.activity_category() {
        SportType::Ride => "󰓅 Speed",
        _ => "👣 Pace",
    };
    let header = vec![
        "#",
        speed_header,
        "🌄",
    ];

    let mut count = 0;
    for split in splits {
        count += 1;
        let color = gradient(
            Rgb { red: 0, green: 255, blue: 0 },
            Rgb { red: 255, green: 0, blue: 0 },
            split.seconds_per_meter() - min,
            max - min,
        ).to_color();
        rows.push(
            Row::new([
                Cell::from(format!("{}", count)).set_style(Style::default().bg(color)),
                match activity.activity_category() {
                    SportType::Ride => Cell::from(
                        app.unit_formatter.speed(split.meters_per_hour()),
                    ),
                    _ => Cell::from(app.unit_formatter.pace(split.moving_time, split.distance))
                },
                Cell::from(app.unit_formatter.elevation(split.elevation_difference)),
            ]),
        );
    }
    Table::new(rows, &[
            Constraint::Min(3),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .header(
            Row::new(header)
                .height(1)
                .bottom_margin(0)
                .style(Style::default()),

        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("")
        .render(area, f, &mut app.activity_view_state.pace_table_state);
}
