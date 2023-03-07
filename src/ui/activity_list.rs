use tui::{
    backend::Backend,
    layout::Constraint,
    style::{Color, Style},
    text::Span,
    widgets::{Cell, Row, Table},
    Frame,
};

use crate::{
    store::activity::ActivityStore,
    util::time_format::{distance, elevation, pace, stopwatch_time, DistanceUnit},
};

use super::unit_formatter::UnitFormatter;

pub struct ActivityList {
    activity_store: ActivityStore,
    unit_formatter: UnitFormatter,
}

impl ActivityList {
    pub fn draw<B: Backend>(
        &self,
        f: &mut Frame<B>,
        area: tui::layout::Rect,
    ) -> Result<(), anyhow::Error> {
        let mut rows = vec![];
        let header_names = ["Date", "", "Title", "Dst", "🕑", "👣", "💓", "🌄"];
        let headers = header_names
            .iter()
            .map(|header| Cell::from(Span::styled(*header, Style::default().fg(Color::DarkGray))));

        for activity in self.activity_store.activities() {
            rows.push(Row::new([
                Cell::from(activity.start_date.format("%Y-%m-%d").to_string()),
                Cell::from(match activity.activity_type.as_str() {
                    "Ride" => "🚴".to_string(),
                    "Run" => "🏃".to_string(),
                    _ => activity.activity_type.clone(),
                }),
                Cell::from(activity.name.clone()),
                Cell::from(self.unit_formatter.distance(activity.distance)),
                Cell::from(self.unit_formatter.stopwatch_time(activity.moving_time)),
                Cell::from(
                    self.unit_formatter
                        .pace(activity.moving_time, activity.distance),
                ),
                Cell::from(
                    activity
                        .average_heartrate
                        .map_or_else(|| "n/a".to_string(), |v| v.to_string()),
                ),
                Cell::from(self.unit_formatter.elevation(activity.total_elevation_gain)),
            ]));
        }

        let table = Table::new(rows)
            .header(
                Row::new(headers)
                    .height(1)
                    .bottom_margin(1)
                    .style(Style::default()),
            )
            .widths(&[
                Constraint::Percentage(10),
                Constraint::Min(2),
                Constraint::Percentage(20),
                Constraint::Percentage(10),
                Constraint::Min(10),
                Constraint::Min(10),
                Constraint::Min(7),
                Constraint::Min(7),
            ]);

        f.render_widget(table, area);
        Ok(())
    }

    pub(crate) fn new(activity_store: ActivityStore) -> Self {
        Self { activity_store, unit_formatter: UnitFormatter::imperial() }
    }
}
