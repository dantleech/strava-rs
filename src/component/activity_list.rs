use tui::{
    backend::Backend,
    layout::Constraint,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Cell, Row, Table, TableState},
    Frame,
};

use crate::{
    store::activity::{ActivityStore, Activity},
};

use super::{event::StravaEvent, unit_formatter::UnitFormatter, layout::State};

pub struct ActivityList<'a> {
    activity_store: &'a mut ActivityStore<'a>,
    unit_formatter: UnitFormatter,
    table_state: TableState,
    activities: Vec<Activity>,
}

impl ActivityList<'_> {
    pub fn handle<'a>(&'a mut self, event: StravaEvent, state: State) -> Option<State> {
        match event {
            StravaEvent::ToggleUnitSystem => {
                self.unit_formatter = self.unit_formatter.toggle();
                None
            },
            StravaEvent::Down => {
                self.next_row();
                None
            },
            StravaEvent::Up => {
                self.prev_row();
                None
            },
            StravaEvent::Enter => {
                return self.select(state);
            },
        }
    }

    pub fn draw<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        area: tui::layout::Rect,
    ) -> Result<(), anyhow::Error> {
        let mut rows = vec![];
        let header_names = ["Date", "", "Title", "Dst", "🕑", "🚤", "👣", "💓", "🌄"];
        let headers = header_names
            .iter()
            .map(|header| Cell::from(Span::styled(*header, Style::default().fg(Color::DarkGray))));

        self.activities = self.activity_store.activities();
        let activities = &self.activities;
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
                Cell::from(self.unit_formatter.distance(activity.distance)),
                Cell::from(self.unit_formatter.stopwatch_time(activity.moving_time)),
                Cell::from(
                    self.unit_formatter
                        .speed(activity.distance, activity.moving_time),
                ),
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

        f.render_stateful_widget(table, area, &mut self.table_state);
        Ok(())
    }

    pub(crate) fn new<'a>(activity_store: &'a mut ActivityStore<'a>) -> ActivityList<'a> {
        ActivityList {
            activities: vec![],
            activity_store,
            unit_formatter: UnitFormatter::imperial(),
            table_state: TableState::default(),
        }
    }

    fn next_row(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => i + 1,
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    fn prev_row(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i > 0 {
                    i - 1
                } else {
                    0
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    fn select<'a>(&'a self, mut state: State) -> Option<State> {
        if let Some(selected) = self.table_state.selected() {
            state.view = super::layout::View::Activity;
            state.activity = Some(self.activities.get(selected).unwrap().clone());
            return Some(state);
        }
        None
    }
}
