use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Cell, Row, Table, TableState},
    Frame,
};

use crate::{
    store::{activity::{ActivityStore, Activity}, self},
};

use super::{event::StravaEvent, unit_formatter::UnitFormatter, layout::State, race_predictor::RacePredictor};

pub struct ActivityView<'a> {
    pub activity: Option<Activity>,
    race_predictor: RacePredictor<'a>,
    unit_formatter: &'a mut UnitFormatter,
}

impl ActivityView<'_> {
    pub fn handle<'a>(&'a mut self, event: StravaEvent, state: State) -> Option<State> {
        match event {
            StravaEvent::ToggleUnitSystem => {
                self.unit_formatter.toggle();
                None
            },
            _ => {
                return self.select(state);
            },
        }
    }

    pub fn draw<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        area: tui::layout::Rect,
    ) -> Result<(), anyhow::Error> {
        let rows = Layout::default()
            .margin(0)
            .constraints([Constraint::Length(1), Constraint::Min(4)].as_ref())
            .split(f.size());
        self.race_predictor.activity = self.activity.clone();
        self.race_predictor.draw(f, rows[0]);
        Ok(())
    }

    pub(crate) fn new(activity: Option<Activity>, unit_formatter: &UnitFormatter) -> ActivityView {
        ActivityView {
            activity,
            unit_formatter,
            race_predictor: RacePredictor::new(&unit_formatter),
        }
    }

    fn select<'a>(&'a self, mut state: State) -> Option<State> {
        state.view = super::layout::View::ActivityList;
        state.activity = None;
        return Some(state);
    }
}

