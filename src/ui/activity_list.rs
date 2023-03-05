use tui::{backend::Backend, Frame, widgets::{Cell, Table, Row}, text::Span, style::{Style, Color}, layout::Constraint};

use crate::store::activity::ActivityStore;


pub struct ActivityList {
    activity_store: ActivityStore,
}

impl ActivityList {
    pub fn draw<B: Backend>(&self, f: &mut Frame<B>, area: tui::layout::Rect) -> Result<(), anyhow::Error> {
        let mut rows = vec![];
        let header_names = ["Date", "Type", "Title", "Dst", "🕑", "💓", "🌄"];
        let headers = header_names
            .iter()
            .map(|header| Cell::from(Span::styled(*header, Style::default().fg(Color::DarkGray))));

        for activity in self.activity_store.activities() {
            rows.push(Row::new([
                Cell::from(activity.start_date.format("%Y-%m-%d").to_string()),
                Cell::from(activity.activity_type.clone()),
                Cell::from(activity.name.clone()),
                Cell::from(activity.distance.to_string()),
                Cell::from(activity.elapsed_time.to_string()),
                Cell::from(activity.average_heartrate.map_or_else(||"n/a".to_string(), |v|v.to_string())),
                Cell::from(activity.total_elevation_gain.to_string()),
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
                Constraint::Percentage(10),
                Constraint::Percentage(20),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
            ]);

        f.render_widget(table, area);
        Ok(())
    }

    pub(crate) fn new(activity_store: ActivityStore) -> Self {
        Self{activity_store}
    }
}
