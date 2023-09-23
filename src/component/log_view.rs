use tui::{widgets::{block::Block, Borders, Widget}, style::{Style, Color}};
use tui_logger::{TuiLoggerWidget, TuiLoggerLevelOutput, TuiWidgetState};

use crate::{event::keymap::StravaEvent, app::ActivePage};

use super::View;

pub struct LogView {
    state: TuiWidgetState
}

impl View for LogView {
    fn handle(&mut self, app: &mut crate::app::App, key: crate::event::keymap::MappedKey) {
        match key.strava_event {
            StravaEvent::Quit => app.quit = true,
            StravaEvent::ToggleLogView => app.active_page = ActivePage::ActivityList,
            _ => (),
        }
    }

    fn draw(
        &mut self,
        _app: &mut crate::app::App,
        f: &mut tui::prelude::Buffer,
        area: tui::layout::Rect,
    ) {
        let tui_w: TuiLoggerWidget = TuiLoggerWidget::default()
            .block(
                Block::default()
                    .title("Independent Tui Logger View")
                    .border_style(Style::default().fg(Color::White).bg(Color::Black))
                    .borders(Borders::ALL),
            )
            .output_separator('|')
            .output_timestamp(Some("%F %H:%M:%S%.3f".to_string()))
            .output_level(Some(TuiLoggerLevelOutput::Long))
            .output_target(false)
            .output_file(false)
            .output_line(false)
            .style(Style::default().fg(Color::White).bg(Color::Black));
        tui_w.render(area, f);
    }
}
