use tui::{backend::Backend, Frame, widgets::{Clear, Block, Borders}, style::Style};

use crate::{app::App, ui::{centered_rect_absolute, color::ColorTheme}, event::keymap::MappedKey};


pub fn handle(app: &mut App, key: MappedKey)  {
}

pub fn draw<B: Backend>(
    app: &mut App,
    f: &mut Frame<B>,
    area: tui::layout::Rect,
) -> Result<(), anyhow::Error> {
    let rect = centered_rect_absolute(64, 3, f.size());
    f.render_widget(Clear, rect);
    let block = Block::default().title("Sort".to_string()).borders(Borders::ALL).style(Style::default().fg(ColorTheme::Dialog.to_color()));

    f.render_widget(block, rect);

    Ok(())
}
