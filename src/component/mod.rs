use tui::{prelude::{Backend, Buffer}, Frame};

use crate::{app::{ActivePage, App}, event::keymap::MappedKey};

pub mod splits;
pub mod activity_list;
pub mod activity_view;
pub mod polyline;
pub mod race_predictor;
pub mod stats;
pub mod unit_formatter;

fn table_status_select_current(app: &mut App) {
    let activities = app.activities();
    if let Some(selected) = app.activity_list.table_state().selected() {
        if let Some(a) = activities.get(selected) {
            app.activity = Some(a.clone());
            app.active_page = ActivePage::Activity;
        }
    }
}

pub trait View {
    fn handle(&self, app: &mut App, key: MappedKey);
    fn draw(&self, app: &mut App, f: &mut Buffer, area: tui::layout::Rect) -> Result<(), anyhow::Error>;
}
