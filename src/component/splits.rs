use tui::{backend::Backend, text::Text, widgets::{Paragraph, BarChart}, Frame};

use crate::app::App;

pub fn draw<B: Backend>(
    app: &mut App,
    f: &mut Frame<B>,
    area: tui::layout::Rect,
) -> Result<(), anyhow::Error> {
    if app.activity.is_none() {
        return Ok(());
    }
    let activity = app.activity.clone().unwrap();

    let mut text = String::new();
    for split in app.activity_splits(activity) {
        text.push_str(format!(
            "{}: {}\n",
            split.split,
            app.unit_formatter.pace(split.moving_time, split.distance)
        ).as_str());
    }
    f.render_widget(Paragraph::new(Text::from(text)), area);
    Ok(())
}

