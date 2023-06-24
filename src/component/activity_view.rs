use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    Frame,
};

use crate::{app::{App, ActivePage}, event::keymap::{StravaEvent, MappedKey}};

use super::race_predictor;

pub fn handle(app: &mut App, key: MappedKey) {
    match key.strava_event {
        StravaEvent::ToggleUnitSystem => {
            app.unit_formatter = app.unit_formatter.toggle();
        },
        StravaEvent::Quit => {
            app.active_page = ActivePage::ActivityList
        },
        StravaEvent::Enter => {
            app.active_page = ActivePage::ActivityList
        },
        _ => {
            ()
        },
    }
}

pub fn draw<B: Backend>(
    app: &mut App,
    f: &mut Frame<B>,
    area: tui::layout::Rect,
) -> Result<(), anyhow::Error> {
    let rows = Layout::default()
        .margin(0)
        .constraints([Constraint::Length(1), Constraint::Min(4)].as_ref())
        .split(area);
    race_predictor::draw(app, f, rows[1])?;
    Ok(())
}
