use tui::{
    backend::Backend,
    layout::Constraint,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Cell, Row, Table},
    Frame,
};

use crate::app::App;

pub fn draw<B: Backend>(
    app: &mut App,
    f: &mut Frame<B>,
    area: tui::layout::Rect,
) -> Result<(), anyhow::Error> {
    let mut rows = vec![];
    let header_names = ["Distance", "Time"];
    let headers = header_names
        .iter()
        .map(|header| Cell::from(Span::styled(*header, Style::default().fg(Color::DarkGray))));

    if let Some(activity) = &app.activity {
        rows.push(Row::new([
            Cell::from("Marathon"),
            Cell::from(app.unit_formatter.stopwatch_time(activity.time_for_distance(42195.0))),
        ]));
        rows.push(Row::new([
            Cell::from("Half Marathon"),
            Cell::from(app.unit_formatter.stopwatch_time(activity.time_for_distance(21097.5))),
        ]));
        rows.push(Row::new([
            Cell::from("10 miles"),
            Cell::from(app.unit_formatter.stopwatch_time(activity.time_for_distance(16093.0))),
        ]));
        rows.push(Row::new([
            Cell::from("10k"),
            Cell::from(app.unit_formatter.stopwatch_time(activity.time_for_distance(10000.0))),
        ]));
        rows.push(Row::new([
            Cell::from("5k"),
            Cell::from(app.unit_formatter.stopwatch_time(activity.time_for_distance(5000.0))),
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
            Constraint::Min(20),
            Constraint::Min(20),
            Constraint::Percentage(20),
        ]);

    f.render_widget(table, area);
    Ok(())
}