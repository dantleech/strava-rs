use geo_types::LineString;
use geoutils::Location;
use polyline;
use tui::{
    backend::Backend,
    text::{Span, Spans, Text},
    widgets::{
        canvas::{Canvas, Line},
        Block, Borders, Paragraph,
    },
    Frame, style::Style,
};

use crate::{
    app::App,
    ui::color::{gradiant, Rgb, ColorTheme},
};

pub fn draw<B: Backend>(
    app: &mut App,
    f: &mut Frame<B>,
    area: tui::layout::Rect,
) -> Result<(), anyhow::Error> {
    if app.activity.is_none() {
        return Ok(());
    }
    let activity = app.activity.clone().unwrap();
    let stats: Vec<(String, String)> = vec![
        ("GPS Points".to_string(), format!("{}", match activity.polyline() {
            Ok(p) => p.lines().len(),
            Err(_) => 0,
        })),
        ("Kudos".to_string(), format!("{}", activity.kudos)),
        ("Country".to_string(), format!("{}", match activity.location_country {
            Some(c) => c.to_string(),
            None => "".to_string(),
        }))
    ];

    let mut text = String::new();
    for (name, value) in stats {
        text.push_str(format!("{}: {}\n", name, value).as_str());

    }
    f.render_widget(Paragraph::new(Text::from(text)), area);
    Ok(())
}
