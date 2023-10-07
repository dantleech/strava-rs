use tui::{prelude::{Buffer}};

use crate::{app::{ActivePage, App}, event::keymap::{MappedKey, StravaEvent}};

pub mod splits;
pub mod activity_list;
pub mod activity_view;
pub mod polyline;
pub mod race_predictor;
pub mod stats;
pub mod unit_formatter;
pub mod log_view;
pub mod calendar_view;

fn table_status_select_current(app: &mut App) {
    let activities = app.activities();
    if let Some(selected) = app.activity_list.table_state().selected() {
        if let Some(a) = activities.get(selected) {
            app.activity = Some(a.clone());
            app.switch_to(ActivePage::Activity);
        }
    }
}

pub trait View {
    fn handle(&mut self, app: &mut App, key: MappedKey);
    fn draw(&mut self, app: &mut App, f: &mut Buffer, area: tui::layout::Rect);
    fn cursor_position(&self) -> Option<(u16,u16)> {
        None
    }
    fn mapped_events(&self, _app: &App) -> Vec<StravaEvent> {
        vec![]
    }
}
