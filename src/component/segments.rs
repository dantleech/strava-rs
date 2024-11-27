use tui::{
    layout::Constraint,
    prelude::Buffer,
    style::{Modifier, Style, Styled},
    text::Text,
    widgets::{Cell, Paragraph, Row, Table, Widget},
};

use crate::{
    app::App,
    store::activity::{ActivitySegmentEffort, Segment, SportType},
};

pub fn draw(app: &mut App, f: &mut Buffer, area: tui::layout::Rect) {
    if app.activity.is_none() {
        return;
    }
    let activity = app.activity.clone().unwrap();
    let efforts: &Vec<ActivitySegmentEffort> = &activity.segment_efforts.as_ref();
    let mut constraints = vec![];
    constraints.push(Constraint::Max(1));
    constraints.push(Constraint::Max(0));

    let mut rows = vec![];
    let sport_type = match activity.sport_type.as_str() {
        "Ride" => SportType::Ride,
        _ => SportType::Run,
    };
    let speed_header = match sport_type {
        SportType::Ride => "󰓅 Speed",
        SportType::Run => "👣 Pace",
    };
    let header = vec!["#", speed_header, "🏅"];

    for effort in efforts {
        match app.segments.get(&effort.segment_id) {
            Some(segment) => {
                rows.push(Row::new([
                    Cell::from(format!("{}", segment.name)).set_style(Style::default()),
                    match sport_type {
                        _ => Cell::from(app.unit_formatter.pace(effort.moving_time, 1000.0)),
                    },
                    Cell::from(format!("{:?}", effort.pr_rank.unwrap_or(0))),
                ]));
                Some(())
            }
            None => None,
        };
    }
    Table::new(rows, &[
            Constraint::Percentage(50),
            Constraint::Percentage(45),
            Constraint::Percentage(5),
        ])
        .header(
            Row::new(header)
                .height(1)
                .bottom_margin(0)
                .style(Style::default()),

        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("")
        .render(area, f);
}
