pub mod list;
pub mod chart;
pub mod sort_dialog;

use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    Frame, widgets::TableState,
};
use tui_input::Input;

use crate::{app::App, event::keymap::MappedKey};

pub fn handle(app: &mut App, key: MappedKey) {
    list::handle(app, key)
}
pub fn draw<B: Backend>(
    app: &mut App,
    f: &mut Frame<B>,
    area: tui::layout::Rect,
) -> Result<(), anyhow::Error> {
    let rows =
        Layout::default().constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)]).split(area);
    chart::draw(app, f, rows[1])?;
    list::draw(app, f, rows[0])?;
    Ok(())
}

pub struct ActivityListState {
    pub table_state: TableState,
    pub filter_text_area: Input,
    pub filter_dialog: bool,
    pub sort_dialog: bool,
}
