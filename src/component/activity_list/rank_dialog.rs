use crossterm::event::KeyCode;
use strum::IntoEnumIterator;
use tui::{
    backend::Backend,
    style::{Color, Style},
    text::{Span, Line},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::{
    app::App,
    event::{keymap::{MappedKey, StravaEvent}, input::InputEvent},
    ui::{centered_rect_absolute, color::ColorTheme}, store::activity::{SortBy, SortOrder},
};

use super::sort_dialog::sort_option_paragraph;

pub fn handle(app: &mut App, key: MappedKey) {
    let matched = match key.strava_event {
        StravaEvent::Enter => {
            app.activity_list.rank_dialog = false;
            true
        },
        StravaEvent::Escape => {
            app.activity_list.rank_dialog = false;
            true
        }
        _ => false,
    };

    if matched {
        return;
    }

    if let Some(sort) = SortBy::from_key(key.key_event.code) {
        app.ranking.rank_by = sort;
        app.ranking.rank_order = match app.ranking.rank_by {
            SortBy::Time => SortOrder::Asc,
            _ => SortOrder::Desc,
        };
        app.activity_list.rank_dialog = false;
        app.send(InputEvent::Reload);
    }
}

pub fn draw<B: Backend>(
    app: &mut App,
    f: &mut Frame<B>,
    area: tui::layout::Rect,
) -> Result<(), anyhow::Error> {
    let rect = centered_rect_absolute(64, 3, area);
    f.render_widget(Clear, rect);
    f.render_widget(sort_option_paragraph(app, "Rank".to_string()), rect);

    Ok(())
}
