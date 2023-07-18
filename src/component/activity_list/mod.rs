pub mod activity_list;
pub mod activity_chart;
pub mod sort_dialog;

use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    Frame, widgets::TableState,
};
use tui_textarea::TextArea;

use crate::{app::App, event::keymap::MappedKey};

pub fn handle(app: &mut App, key: MappedKey) {
    activity_list::handle(app, key)
}
pub fn draw<B: Backend>(
    app: &mut App,
    f: &mut Frame<B>,
    area: tui::layout::Rect,
) -> Result<(), anyhow::Error> {
    let rows =
        Layout::default().constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)]).split(area);
    activity_list::draw(app, f, rows[0])?;
    activity_chart::draw(app, f, rows[1])?;
    Ok(())
}

pub struct ActivityListState<'a> {
    pub table_state: TableState,
    pub filter_text_area: TextArea<'a>,
    pub filter_dialog: bool,
    pub sort_dialog: bool,
}

