use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    widgets::Paragraph,
    Frame,
};

use super::activity_list::ActivityList;

pub struct AppLayout {
    activities_list: ActivityList,
}

impl AppLayout {
    pub fn draw<B: Backend>(&self, f: &mut Frame<B>) -> Result<(), anyhow::Error> {
        let rows = Layout::default()
            .margin(0)
            .constraints([Constraint::Length(1), Constraint::Min(4)].as_ref())
            .split(f.size());
        f.render_widget(Paragraph::new("Activities"), rows[0]);

        self.activities_list.draw(f, rows[1])?;

        Ok(())
    }

    pub(crate) fn new(activities_list: ActivityList) -> AppLayout {
        AppLayout { activities_list }
    }
}
