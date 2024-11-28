use tui::{
    layout::{Constraint, Direction, Layout, Margin},
    widgets::{Block, Borders, Widget},
};

use crate::{app::{ActivePage, App}, event::{keymap::StravaEvent, util::{table_state_next, table_state_prev}}, store::activity::Activities};

use super::{activity_list::{list::activity_list_table, toggle_moving_elapsed}, segments, View};

pub struct ActivitySegments {}

impl ActivitySegments {
    pub(crate) fn new() -> ActivitySegments {
        ActivitySegments {}
    }
}

impl View for ActivitySegments {
    fn handle(&mut self, app: &mut crate::app::App, key: crate::event::keymap::MappedKey) {
        let nb_segments = match &app.activity {
            Some(a) => a.segment_efforts.len(),
            None => 0,
        };
        match key.strava_event {
            StravaEvent::ToggleUnitSystem => {
                app.unit_formatter = app.unit_formatter.toggle();
            }
            StravaEvent::Quit => app.switch_to(ActivePage::ActivityList),
            StravaEvent::ToggleSegmentEffortsView => app.switch_to(ActivePage::Activity),
            StravaEvent::Down => {
                app.next_activity();
            }
            StravaEvent::Up => {
                app.previous_activity();
            }
            StravaEvent::MovingElapsed => toggle_moving_elapsed(app),
            StravaEvent::Next => {
                table_state_next(&mut app.activity_view_state.segment_efforts_state, nb_segments, true);
            }
            StravaEvent::Previous => {
                table_state_prev(&mut app.activity_view_state.segment_efforts_state, nb_segments, true);
            }
            _ => (),
        }
    }

    fn mapped_events(&self, _app: &App) -> Vec<StravaEvent> {
        vec![
            StravaEvent::Up,
            StravaEvent::Down,
            StravaEvent::Next,
            StravaEvent::Previous,
            StravaEvent::ToggleSegmentEffortsView,
            StravaEvent::Quit,
        ]
    }
    fn draw(
        &mut self,
        app: &mut crate::app::App,
        f: &mut tui::prelude::Buffer,
        area: tui::layout::Rect,
    ) {
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(4), Constraint::Min(2)].as_ref())
            .split(area);
        let block = Block::default().title("Segment Efforts").borders(Borders::ALL);
        block.render(rows[1], f);

        if let Some(activity) = &app.activity {
            {
                let a = Activities::from(activity.clone());
                activity_list_table(app, &a).render(rows[0], f);
            }
        }

        segments::draw(
            app,
            f,
            rows[1].inner(Margin {
                vertical: 2,
                horizontal: 2,
            }),
        );
    }
}
