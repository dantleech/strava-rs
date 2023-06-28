use tui::{
    backend::Backend,
    layout::Constraint,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Cell, Clear, Row, Table},
    Frame,
};

use crate::{
    app::{ActivePage, App},
    event::{
        keymap::{MappedKey, StravaEvent},
        util::{table_state_next, table_state_prev},
    },
    store::activity::Activity,
    ui::{centered_rect_absolute, color::ColorTheme, key_event_to_input},
};

use super::table_status_select_current;

pub fn handle(app: &mut App, key: MappedKey) {
    if app.activity_list_filter_dialog == true {
        let matched = match key.strava_event {
            StravaEvent::Enter => {
                app.activity_list_filter =
                    app.activity_list_filter_text_area.lines()[0].to_string();
                app.activity_list_filter_dialog = false;
                app.activity_list_table_state.select(Some(0));
                true
            }
            _ => false,
        };
        if matched {
            return;
        }

        app.activity_list_filter_text_area
            .input(key_event_to_input(key.key_event));
        return;
    }
    let activities = app.filtered_activities();
    match key.strava_event {
        StravaEvent::Quit => app.quit = true,
        StravaEvent::ToggleUnitSystem => {
            app.unit_formatter = app.unit_formatter.toggle();
        }
        StravaEvent::Down => table_state_next(&mut app.activity_list_table_state, activities.len()),
        StravaEvent::Up => table_state_prev(&mut app.activity_list_table_state, activities.len()),
        StravaEvent::Filter => toggle_filter(app),
        StravaEvent::Enter => table_status_select_current(app),
        _ => (),
    }
}

fn toggle_filter(app: &mut App) {
    app.activity_list_filter_dialog = !app.activity_list_filter_dialog;
}

pub fn draw<B: Backend>(
    app: &mut App,
    f: &mut Frame<B>,
    area: tui::layout::Rect,
) -> Result<(), anyhow::Error> {
    let activities = &app.filtered_activities();

    if app.activity_list_table_state.selected() == None && activities.len() > 0 {
        app.activity_list_table_state.select(Some(0));
    }

    f.render_stateful_widget(
        activity_list_table(app, activities),
        area,
        &mut app.activity_list_table_state,
    );

    if app.activity_list_filter_dialog == true {
        let rect = centered_rect_absolute(64, 3, f.size());
        app.activity_list_filter_text_area.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title("Filter")
                .border_style(Style::default().fg(ColorTheme::Orange.to_color())),
        );

        f.render_widget(Clear, rect);
        f.render_widget(app.activity_list_filter_text_area.widget(), rect);
    }

    Ok(())
}

pub fn activity_list_table<'a>(app: &App, activities: &'a Vec<Activity>) -> Table<'a> {
    let mut rows = vec![];
    let header_names = [
        "Date",
        "",
        "Title",
        "Dst",
        "🕑 Time",
        "👣 Pace",
        "💓 Heart",
        "🌄 Elevation",
    ];
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
        .highlight_symbol("")
        .widths(&[
            Constraint::Min(10),
            Constraint::Min(2),
            Constraint::Percentage(20),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
        ])
}
