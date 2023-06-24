use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    widgets::Paragraph,
    Frame,
};

use crate::store::activity::Activity;

use super::activity_list::ActivityList;

#[derive(Debug, Clone)]
pub enum View {
    ActivityList,
    Activity,
}

#[derive(Debug, Clone)]
pub struct State {
    pub view: View,
    pub activity: Option<Activity>,
}

pub struct AppLayout<'a> {
    activities_list: &'a mut ActivityList<'a>,
    state: State,
}

impl AppLayout<'_> {
    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>) -> Result<(), anyhow::Error> {
        match self.state.view {
            _ => {
                let rows = Layout::default()
                    .margin(0)
                    .constraints([Constraint::Length(1), Constraint::Min(4)].as_ref())
                    .split(f.size());
                f.render_widget(Paragraph::new("Activities"), rows[0]);

                self.activities_list.draw(f, rows[1])?;
            }
        }

        Ok(())
    }

    pub(crate) fn new<'a>(
        activities_list: &'a mut ActivityList<'a>,
        state: State,
    ) -> AppLayout<'a> {
        AppLayout {
            activities_list,
            state,
        }
    }

    pub(crate) fn handle<'a, 'b>(&'a mut self, event: super::event::StravaEvent) {
        if let Some(state) = self.activities_list.handle(event, self.state.clone()) {
            self.state = state
        }
    }
}
