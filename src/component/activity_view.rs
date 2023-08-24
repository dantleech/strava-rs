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
        util::{table_state_next, table_state_prev}, input::InputEvent,
    },
};

use super::{
    polyline, race_predictor, stats,
    table_status_select_current, splits, activity_list::list::activity_list_table,
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
            table_state_next(&mut app.activity_list.table_state(), activities.len());
            table_status_select_current(app);
        },
        StravaEvent::Up => {
            table_state_prev(&mut app.activity_list.table_state(), activities.len());
            table_status_select_current(app);
        },
        StravaEvent::Anchor => {
            app.anchor_selected();
            app.send(InputEvent::Reload);
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
        .constraints([Constraint::Percentage(25), Constraint::Percentage(50), Constraint::Percentage(25)].as_ref())
        .split(rows[1]);
    let col1 = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(cols[0]);

    let block = Block::default()
        .title("Race Predictions")
        .borders(Borders::ALL);
    f.render_widget(block, col1[0]);
    race_predictor::draw(
        app,
        f,
        col1[0].inner(&Margin {
            vertical: 2,
            horizontal: 2,
        }),
    )?;

    let block = Block::default().title("Stats").borders(Borders::ALL);
    f.render_widget(block, col1[1]);
    stats::draw(
        app,
        f,
        col1[1].inner(&Margin {
            vertical: 1,
            horizontal: 1,
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
    let block = Block::default().title("Splits").borders(Borders::ALL);
    f.render_widget(block, cols[2]);
    splits::draw(
        app,
        f,
        cols[2].inner(&Margin {
            vertical: 1,
            horizontal: 1,
        }),
    )?;

    Ok(())
}
