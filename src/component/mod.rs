use crate::app::{ActivePage, App};

pub mod splits;
pub mod activity_list;
pub mod activity_view;
pub mod polyline;
pub mod race_predictor;
pub mod stats;
pub mod unit_formatter;

fn table_status_select_current(app: &mut App) {
    let activities = app.filtered_activities();
    if let Some(selected) = app.activity_list.table_state.selected() {
        if let Some(a) = activities.get(selected) {
            app.activity = Some(a.clone());
            app.active_page = ActivePage::Activity;
        }
    }
}
fn table_status_anchor_current(app: &mut App) {
    let activities = app.filtered_activities();
    if let Some(selected) = app.activity_list.table_state.selected() {

        if let Some(a) = activities.get(selected) {

            if app.activity_anchored.is_some() {
                app.activity_anchored = None;
                return;
            }

            app.activity_anchored = Some(a.clone());
        }
        app.activity_list.table_state.select(Some(0));
    }
}
