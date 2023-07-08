
use std::f32::MAX;

use diesel::internal::derives::multiconnection::array_comparison::AsInExpression;
use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{BarChart, Gauge, Paragraph, Block, Borders},
    Frame,
};

use crate::{app::App, ui::color::{ColorTheme, gradiant, Rgb}};

pub fn draw<B: Backend>(
    app: &mut App,
    f: &mut Frame<B>,
    area: tui::layout::Rect,
) -> Result<(), anyhow::Error> {
    if app.activity.is_none() {
        return Ok(());
    }
    let activity = app.activity.clone().unwrap();

    let splits = app.activity_splits(activity);
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

    let rows = Layout::default().constraints(constraints).split(area);
    let master_cols = Layout::default().direction(tui::layout::Direction::Horizontal).constraints(vec![
         Constraint::Length(3),
         Constraint::Min(1),
         Constraint::Length(10),
    ]);
    let header = master_cols.split(area);
    f.render_widget(Paragraph::new("#"), header[0]);
    f.render_widget(Paragraph::new("👣 Pace"), header[1]);
    f.render_widget(Paragraph::new("🌄"), header[2]);

    let mut count = 1;
    let split_count = splits.len();
    for split in splits {
        let row = rows[count];
        let cols = master_cols.split(row);

        let percent = ((((split.seconds_per_meter() - min) as f32 / (max - min) as f32) * 100.0) * 0.75) as u16;
        f.render_widget(Paragraph::new(format!("{}", count)), cols[0]);
        f.render_widget(
            Gauge::default()
                .percent(percent + 25)
                .label(
                    app.unit_formatter.pace(split.moving_time, split.distance),
                )
                .use_unicode(true)
                .style(Style::default().fg(Color::White))
                .gauge_style(Style::default().fg(
                        gradiant(
                            Rgb { red: 0, green: 255, blue: 0 },
                            Rgb { red: 255, green: 0, blue: 0 },
                            (split.seconds_per_meter() - min) as f64,
                            (max - min) as f64,
                        ).to_color()
                ).bg(Color::Black)),

            cols[1],
        );
        f.render_widget(Paragraph::new(app.unit_formatter.elevation(split.elevation_difference)), cols[2]);
        count += 1;
    }
    Ok(())
}
