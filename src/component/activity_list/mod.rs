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

pub enum ActivityListMode {
    Normal,
    Anchored,
}

pub struct ActivityListState {
    pub mode: ActivityListMode,
    pub table_state: TableState,
    pub anchored_table_state: TableState,
    pub filter_text_area: Input,
    pub filter_dialog: bool,
    pub sort_dialog: bool,
}

pub struct ActivityViewState {
    pub pace_table_state: TableState,
    pub selected_split: Option<i64>,
}
impl ActivityViewState {
    pub(crate) fn select_split(&mut self, selected: i64) -> () {
        self.selected_split = Some(selected);
    }
}

impl ActivityListState {
    pub fn table_state(&mut self) -> &mut TableState
    {
        match self.mode {
            ActivityListMode::Normal => &mut self.table_state,
            ActivityListMode::Anchored => &mut self.anchored_table_state,
        }
    }
}
