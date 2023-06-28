use crate::app::{App, ActivePage};

pub mod activity_list;
pub mod activity_view;
pub mod unit_formatter;
pub mod race_predictor;
pub mod polyline;

fn table_status_select_current(app:  &mut App) {
    let activities = app.filtered_activities();
    if let Some(selected) = app.activity_list_table_state.selected() {
        if let Some(a) = activities.get(selected) {
            app.activity = Some(a.clone());
            app.active_page = ActivePage::Activity;
        }
    }
}

