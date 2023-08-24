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

use super::activity_list::list::activity_list_table;



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
    app: &mut App,
    f: &mut Frame<B>,
    area: tui::layout::Rect,
) -> Result<(), anyhow::Error> {
    if app.activity.is_none() {
        return Ok(());
    }
    // todo: refactor to collection
    let activities = app.similar_activities(app.activity.clone().unwrap());

    f.render_stateful_widget(
        activity_list_table(app, &activities),
        area,
        &mut app.activity_performances.table_state,
    );

    Ok(())
}

