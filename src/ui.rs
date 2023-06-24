use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    widgets::Paragraph,
    Frame,
};

use crate::{app::{App, ActivePage}, component::{activity_list, activity_view}};

pub fn draw<B: Backend>(app: &mut App, f: &mut Frame<B>) -> Result<(), anyhow::Error> {
    let rows = Layout::default()
        .margin(0)
        .constraints([Constraint::Min(4)].as_ref())
        .split(f.size());

    match app.active_page {
        ActivePage::ActivityList => {
            activity_list::draw(app, f, rows[0])?;
        }
        ActivePage::Activity => {
            activity_view::draw(app, f, rows[0])?;
        }
    }

    Ok(())
}
