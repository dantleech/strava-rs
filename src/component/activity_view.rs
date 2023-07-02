use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Margin},
    widgets::{Block, Borders},
    Frame,
};

use crate::{
    app::{ActivePage, App},
    event::{
        keymap::{MappedKey, StravaEvent},
        util::{table_state_next, table_state_prev},
    },
};

use super::{
    activity_list::activity_list_table, polyline, race_predictor, table_status_select_current,
};

pub fn handle(app: &mut App, key: MappedKey) {
    let activities = app.filtered_activities();
    match key.strava_event {
        StravaEvent::ToggleUnitSystem => {
            app.unit_formatter = app.unit_formatter.toggle();
        }
        StravaEvent::Quit => app.active_page = ActivePage::ActivityList,
        StravaEvent::Enter => app.active_page = ActivePage::ActivityList,
        StravaEvent::Down => {
            table_state_next(&mut app.activity_list_table_state, activities.len());
            table_status_select_current(app);
        }
        StravaEvent::Up => {
            table_state_prev(&mut app.activity_list_table_state, activities.len());
            table_status_select_current(app);
        }
        _ => (),
    }
}

pub fn draw<B: Backend>(
    app: &mut App,
    f: &mut Frame<B>,
    area: tui::layout::Rect,
) -> Result<(), anyhow::Error> {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(4), Constraint::Length(2)].as_ref())
        .split(area);

    if let Some(activity) = &app.activity {
        f.render_widget(activity_list_table(app, &vec![activity.clone()]), rows[0]);
    }

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
        .split(rows[1]);

    let block = Block::default()
        .title("Race Predictions")
        .borders(Borders::ALL);

    f.render_widget(block, cols[0]);

    race_predictor::draw(
        app,
        f,
        cols[0].inner(&Margin {
            vertical: 2,
            horizontal: 2,
        }),
    )?;

    let block = Block::default().title("Map").borders(Borders::ALL);

    f.render_widget(block, cols[1]);
    polyline::draw(
        app,
        f,
        cols[1].inner(&Margin {
            vertical: 1,
            horizontal: 1,
        }),
    )?;

    Ok(())
}
