pub mod sort_dialog;
pub mod activity_list;

use tui::{
    backend::Backend,
    layout::Constraint,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Cell, Clear, Row, Table},
    Frame,
};

use crate::{
    app::{App, SortOrder},
    event::{
        keymap::{MappedKey, StravaEvent},
        util::{table_state_next, table_state_prev},
    },
    store::activity::Activity,
    ui::{centered_rect_absolute, color::ColorTheme, key_event_to_input},
};

pub fn handle(app: &mut App, key: MappedKey) {
    activity_list::handle(app, key)
}
pub fn draw<B: Backend>(
    app: &mut App,
    f: &mut Frame<B>,
    area: tui::layout::Rect,
) -> Result<(), anyhow::Error> {
    activity_list::draw(app, f, area);
    Ok(())
}
