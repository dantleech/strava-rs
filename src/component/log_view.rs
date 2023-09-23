use tui::{widgets::{block::Block, Borders, Widget}, style::{Style, Color}};
use tui_logger::{TuiLoggerWidget, TuiLoggerLevelOutput, TuiWidgetState};

use crate::{event::keymap::StravaEvent, app::{ActivePage, App}};

use super::View;

pub struct LogView {
    state: TuiWidgetState
}
impl LogView {
    pub(crate) fn new() -> LogView {
        LogView{ state: TuiWidgetState::default() }
    }
}

impl View for LogView {
    fn mapped_events(&self, _app: &App) -> Vec<StravaEvent> {
        vec![
            StravaEvent::ToggleLogView,
            StravaEvent::Quit,
        ]
    }
    fn handle(&mut self, app: &mut crate::app::App, key: crate::event::keymap::MappedKey) {
        match key.strava_event {
            StravaEvent::Quit => app.quit = true,
            StravaEvent::ToggleLogView => app.switch_to_previous(),
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
