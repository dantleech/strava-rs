use tui::{
    backend::Backend,
    Frame,
};

use crate::{
    app::{ActivePage, App},
    event::{
        keymap::{MappedKey, StravaEvent},
    },
};



pub fn handle(app: &mut App, key: MappedKey) {
    match key.strava_event {
        StravaEvent::ToggleUnitSystem => {
            app.unit_formatter = app.unit_formatter.toggle();
        }
        StravaEvent::Quit => app.active_page = ActivePage::Activity,
        StravaEvent::Enter => app.active_page = ActivePage::Activity,
        StravaEvent::AlternateView => {
            app.active_page = ActivePage::Activity;
        }
        _ => (),
    }
}

pub fn draw<B: Backend>(
    _app: &mut App,
    _f: &mut Frame<B>,
    _area: tui::layout::Rect,
) -> Result<(), anyhow::Error> {

    Ok(())
}

