use tui::{backend::Backend, Frame, widgets::{Cell, Table, Row}, text::Span, style::{Style, Color}, layout::Constraint};

use crate::{store::activity::ActivityStore, util::time_format::{stopwatch_time, distance, DistanceUnit, pace, elevation}};


pub struct ActivityList {
    activity_store: ActivityStore,
}

impl ActivityList {
    pub fn draw<B: Backend>(&self, f: &mut Frame<B>, area: tui::layout::Rect) -> Result<(), anyhow::Error> {
        let mut rows = vec![];
        let header_names = ["Date", "", "Title", "Dst", "🕑", "👣", "💓", "🏔"];
        let headers = header_names
            .iter()
            .map(|header| Cell::from(Span::styled(*header, Style::default().fg(Color::DarkGray))));
        let unit =DistanceUnit::Imperial;

        for activity in self.activity_store.activities() {
            rows.push(Row::new([
                Cell::from(activity.start_date.format("%Y-%m-%d").to_string()),
                Cell::from(match activity.activity_type.as_str() {
                    "Ride" => "🚴".to_string(),
                    "Run" => "🏃".to_string(),
                    _ => activity.activity_type.clone(),
                }),
                Cell::from(activity.name.clone()),
                Cell::from(distance(activity.distance, &unit)),
                Cell::from(stopwatch_time(activity.moving_time)),
                Cell::from(pace(activity.moving_time, activity.distance, &unit)),
                Cell::from(activity.average_heartrate.map_or_else(||"n/a".to_string(), |v|v.to_string())),
                Cell::from(elevation(activity.total_elevation_gain, &unit)),
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
        Self{activity_store}
    }
}
