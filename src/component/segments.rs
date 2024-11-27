use tui::{
    layout::Constraint,
    prelude::Buffer,
    style::{Modifier, Style, Styled},
    widgets::{Cell, Row, Table, Widget},
};

use crate::{
    app::App,
    store::activity::{ActivitySegmentEffort},
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
    let header = vec!["Name", ""];

    for effort in efforts {
        match app.segments.get(&effort.segment_id) {
            Some(segment) => {
                rows.push(Row::new([
                    Cell::from(format!("{}", segment.name)).set_style(Style::default()),
                    Cell::from(format!("{}", match effort.pr_rank {
                        Some(1) => "🥇",
                        Some(2) => "🥈",
                        Some(3) => "🥉",
                        _ => ""
                    })),
                ]));
                Some(())
            }
            None => None,
        };
    }
    Table::new(rows, &[
            Constraint::Min(5),
            Constraint::Length(2),
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
