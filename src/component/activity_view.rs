use tui::{
    layout::{Constraint, Direction, Layout, Margin},
    prelude::Buffer,
    widgets::{Block, Borders, Widget},
};

use crate::{
    app::{ActivePage, App},
    event::{
        input::InputEvent,
        keymap::{MappedKey, StravaEvent},
        util::{table_state_next, table_state_prev},
    },
    store::activity::Activities,
};

use super::{
    activity_list::list::activity_list_table, polyline, race_predictor, splits, stats, View,
};

pub struct ActivityView {}

impl View for ActivityView {
    fn handle(&mut self, app: &mut App, key: MappedKey) {
        let split_len = match &app.activity {
            Some(a) => a.splits.len(),
            None => 0,
        };

        match key.strava_event {
            StravaEvent::ToggleUnitSystem => {
                app.unit_formatter = app.unit_formatter.toggle();
            }
            StravaEvent::Quit => app.switch_to(ActivePage::ActivityList),
            StravaEvent::Enter => app.switch_to(ActivePage::ActivityList),
            StravaEvent::Down => {
                app.next_activity();
            }
            StravaEvent::Up => {
                app.previous_activity();
            }
            StravaEvent::Next => {
                table_state_next(&mut app.activity_view_state.pace_table_state, split_len, true);
                if let Some(selected) = app.activity_view_state.pace_table_state.selected() {
                    app.activity_view_state.select_split(selected as i64);
                }
            }
            StravaEvent::Previous => {
                table_state_prev(&mut app.activity_view_state.pace_table_state, split_len, true);
                if let Some(selected) = app.activity_view_state.pace_table_state.selected() {
                    app.activity_view_state.select_split(selected as i64);
                }
            }
            StravaEvent::Anchor => {
                app.anchor_selected();
                app.send(InputEvent::Reload);
            }
            StravaEvent::ToggleLogView => {
                app.switch_to(ActivePage::LogView);
            }
            _ => (),
        }
    }

    fn mapped_events(&self, _app: &App) -> Vec<StravaEvent> {
        vec![
            StravaEvent::Down,
            StravaEvent::Up,
            StravaEvent::ToggleUnitSystem,
            StravaEvent::Next,
            StravaEvent::Previous,
            StravaEvent::Enter,
            StravaEvent::ToggleLogView,
            StravaEvent::Quit,
        ]
    }

    fn draw(&mut self, app: &mut App, f: &mut Buffer, area: tui::layout::Rect) {
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(4), Constraint::Min(2)].as_ref())
            .split(area);

        if let Some(activity) = &app.activity {
            {
                let a = Activities::from(activity.clone());
                activity_list_table(app, &a).render(rows[0], f);
            }
        }

        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(25),
                    Constraint::Percentage(50),
                    Constraint::Percentage(25),
                ]
                .as_ref(),
            )
            .split(rows[1]);
        let col1 = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(cols[0]);

        let block = Block::default()
            .title("Race Predictions")
            .borders(Borders::ALL);
        block.render(col1[0], f);

        race_predictor::draw(
            app,
            f,
            col1[0].inner(Margin {
                vertical: 2,
                horizontal: 2,
            }),
        );

        let block = Block::default().title("Stats").borders(Borders::ALL);
        block.render(col1[1], f);

        stats::draw(
            app,
            f,
            col1[1].inner(Margin {
                vertical: 1,
                horizontal: 1,
            }),
        );

        let block = Block::default().title("Map").borders(Borders::ALL);
        block.render(cols[1], f);
        polyline::draw(
            app,
            f,
            cols[1].inner(Margin {
                vertical: 1,
                horizontal: 1,
            }),
        );
        let block = Block::default().title("Splits").borders(Borders::ALL);
        block.render(cols[2], f);
        splits::draw(
            app,
            f,
            cols[2].inner(Margin {
                vertical: 1,
                horizontal: 1,
            }),
        );
    }
}
