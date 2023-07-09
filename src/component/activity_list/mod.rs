pub mod activity_list;
pub mod sort_dialog;

use tui::{backend::Backend, Frame};

use crate::{app::App, event::keymap::MappedKey};

pub fn handle(app: &mut App, key: MappedKey) {
    activity_list::handle(app, key)
}
pub fn draw<B: Backend>(
    app: &mut App,
    f: &mut Frame<B>,
    area: tui::layout::Rect,
) -> Result<(), anyhow::Error> {
    activity_list::draw(app, f, area)?;
    Ok(())
}
