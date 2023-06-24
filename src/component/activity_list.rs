use tui::{
    backend::Backend,
    layout::Constraint,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Cell, Row, Table},
    Frame,
};

use crate::{
    app::{ActivePage, App},
    event::{
        keymap::{MappedKey, StravaEvent},
        util::{table_state_next, table_state_prev},
    }, store::activity::Activity,
};

pub fn handle(app: &mut App, key: MappedKey) {
    match key.strava_event {
        StravaEvent::Quit => app.quit = true,
        StravaEvent::ToggleUnitSystem => {
            app.unit_formatter = app.unit_formatter.toggle();
        }
        StravaEvent::Down => {
            table_state_next(&mut app.activity_list_table_state, app.activities.len())
        }
        StravaEvent::Up => {
            table_state_prev(&mut app.activity_list_table_state, app.activities.len())
        }
        StravaEvent::Enter => {
            if let Some(selected) = app.activity_list_table_state.selected() {
                app.activity = Some(app.activities.get(selected).unwrap().clone());
                app.active_page = ActivePage::Activity;
            }
        },
        _ => (),
    }
}

pub fn draw<B: Backend>(
    app: &mut App,
    f: &mut Frame<B>,
    area: tui::layout::Rect,
) -> Result<(), anyhow::Error> {
    let activities = &app.activities;

    if app.activity_list_table_state.selected() == None && activities.len() > 0 {
        app.activity_list_table_state.select(Some(0));
    }

    f.render_stateful_widget(activity_list_table(app, activities), area, &mut app.activity_list_table_state);
    Ok(())
}

pub fn activity_list_table<'a>(app: &App, activities: &'a Vec<Activity>) -> Table<'a> {
    let mut rows = vec![];
    let header_names = ["Date", "", "Title", "Dst", "🕑 Time", "👣 Pace", "💓 Heart", "🌄 Elevation"];
    let headers = header_names
        .iter()
        .map(|header| Cell::from(Span::styled(*header, Style::default().fg(Color::DarkGray))));

    for activity in activities {
        rows.push(Row::new([
            Cell::from(match activity.start_date {
                Some(x) => x.format("%Y-%m-%d").to_string(),
                None => "".to_string(),
            }),
            Cell::from(activity.activity_type_icon()),
            Cell::from(activity.title.clone()),
            Cell::from(app.unit_formatter.distance(activity.distance)),
            Cell::from(app.unit_formatter.stopwatch_time(activity.moving_time)),
            Cell::from(
                app.unit_formatter
                    .pace(activity.moving_time, activity.distance),
            ),
            Cell::from(
                activity
                    .average_heartrate
                    .map_or_else(|| "n/a".to_string(), |v| v.to_string()),
            ),
            Cell::from(app.unit_formatter.elevation(activity.total_elevation_gain)),
        ]));
    }

    Table::new(rows)
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
        ])
}
