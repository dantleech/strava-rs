use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    widgets::Paragraph,
    Frame,
};

use crate::{app::{App, ActivePage}, component::activity_list};

pub fn draw<B: Backend>(app: &mut App, f: &mut Frame<B>) -> Result<(), anyhow::Error> {
    let rows = Layout::default()
        .margin(0)
        .constraints([Constraint::Length(1), Constraint::Min(4)].as_ref())
        .split(f.size());
    f.render_widget(Paragraph::new("Activities"), rows[0]);
    match app.active_page {
        ActivePage::ActivityList => {
            activity_list::draw(app, f, rows[1])?;
        }
    }

    Ok(())
}
