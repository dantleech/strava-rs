use tui::{text::Text, widgets::Paragraph, widgets::Widget, prelude::Buffer};

use crate::app::App;

pub fn draw(
    app: &mut App,
    f: &mut Buffer,
    area: tui::layout::Rect,
) {
    if app.activity.is_none() {
        return;
    }
    let activity = app.activity.clone().unwrap();
    let stats: Vec<(String, String)> = vec![
        (
            "GPS Points".to_string(),
            format!(
                "{}",
                match activity.polyline() {
                    Ok(p) => p.lines().len(),
                    Err(_) => 0,
                }
            ),
        ),
        ("Kudos".to_string(), format!("{}", activity.kudos)),
        (
            "Split".to_string(),
            match app.activity_view.selected_split {
                Some(s) => format!("{}", s),
                None => "N/A".to_string()
            },
        ),
        (
            "Country".to_string(),
            (match activity.location_country {
                    Some(c) => c,
                    None => "".to_string(),
                }),
        ),
    ];

    let mut text = String::new();
    if !activity.description.is_empty() {
        text.push_str(format!("{}\n\n", &activity.description).as_str());
    }
    for (name, value) in stats {
        text.push_str(format!("{}: {}\n", name, value).as_str());
    }
    Paragraph::new(Text::from(text)).render(area, f);
}
