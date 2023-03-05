
use tui::{backend::Backend, Frame, layout::{Constraint, Layout}, widgets::Paragraph};

pub struct AppLayout {

}

impl AppLayout {

    pub fn draw<B: Backend>(&self, f: &mut Frame<B>) -> Result<(), anyhow::Error> {
        let rows = Layout::default()
            .margin(0)
            .constraints([
                Constraint::Length(1),
                Constraint::Min(4),
            ].as_ref())
            .split(f.size());
        f.render_widget(Paragraph::new("Activities"), rows[0]);

        Ok(())
    }
}
