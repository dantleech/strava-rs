use tui::{
    layout::Constraint, prelude::Buffer, style::{Color, Modifier, Style, Styled}, text::Span, widgets::{Cell, Row, StatefulWidget, Table}
};

use crate::{app::App, store::activity::ActivitySegmentEffort};

pub fn draw(app: &mut App, f: &mut Buffer, area: tui::layout::Rect) {
    if app.activity.is_none() {
        return;
    }
    let activity = app.activity.clone().unwrap();
    let efforts: &Vec<ActivitySegmentEffort> = activity.segment_efforts.as_ref();

    let mut rows = vec![];
    let header = vec!["🏅", "Name", "Dst", "Time", "👣 Pace", "󰓅 Speed"];
    let header = header
        .iter()
        .map(|header| Cell::from(Span::styled(*header, Style::default().fg(Color::DarkGray))));

    for effort in efforts {
        match app.segments.get(&effort.segment_id) {
            Some(segment) => {
                rows.push(Row::new([
                    Cell::from(
                        (match effort.pr_rank {
                            Some(1) => "🥇1",
                            Some(2) => "🥈2",
                            Some(3) => "🥉3",
                            _ => "",
                        })
                        .to_string(),
                    ),
                    Cell::from(segment.name.to_string()).set_style(Style::default()),
                    Cell::from(app.unit_formatter.distance(segment.distance)),
                    Cell::from(app.unit_formatter.stopwatch_time(
                        match app.activity_list.use_moving_time {
                            true => effort.moving_time,
                            false => effort.elapsed_time,
                        },
                    )),
                    Cell::from(match app.activity_list.use_moving_time {
                        true => app
                            .unit_formatter
                            .pace(effort.moving_time, segment.distance),
                        false => app
                            .unit_formatter
                            .pace(effort.elapsed_time, segment.distance),
                    }),
                    Cell::from(app.unit_formatter.speed(effort.meters_per_hour(segment.distance))),

                ]));
                Some(())
            }
            None => None,
        };
    }
    Table::new(rows, &[
        Constraint::Max(3),
        Constraint::Percentage(40),
        Constraint::Max(8),
        Constraint::Max(8),
        Constraint::Max(8),
        Constraint::Max(8),
    ])
        .header(
            Row::new(header)
                .height(1)
                .bottom_margin(1)
                .style(Style::default()),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("")
        .render(area, f, &mut app.activity_view_state.segment_efforts_state);
}
