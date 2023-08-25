
use std::f64::MAX;


use tui::{
    backend::Backend,
    layout::{Constraint},
    style::{Style, Styled, Modifier},
    widgets::{Row, Cell, Table},
    Frame,
};

use crate::{app::App, ui::color::{gradient, Rgb}, store::activity::ActivitySplit};

pub fn draw<B: Backend>(
    app: &mut App,
    f: &mut Frame<B>,
    area: tui::layout::Rect,
) -> Result<(), anyhow::Error> {
    if app.activity.is_none() {
        return Ok(());
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
    let header = vec![
        "#",
        "👣 Pace",
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
                Cell::from(app.unit_formatter.pace(split.moving_time, split.distance)),
                Cell::from(app.unit_formatter.elevation(split.elevation_difference)),
            ]),
        );
    }
    f.render_stateful_widget(
        Table::new(rows)
            .header(
                Row::new(header)
                    .height(1)
                    .bottom_margin(0)
                    .style(Style::default()),

            ).widths(&[
                Constraint::Min(3),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ])
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("")
            , area, &mut app.activity_view.pace_table_state
    );
    Ok(())
}
