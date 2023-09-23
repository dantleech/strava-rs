use log::{LevelFilter, info, warn};
use tui::{
    style::{Color, Style},
    widgets::{block::Block, Borders, Widget},
};
use tui_logger::{TuiLoggerLevelOutput, TuiLoggerWidget, TuiWidgetState, TuiLoggerSmartWidget};

use crate::{
    app::{ActivePage, App},
    event::keymap::StravaEvent,
};

use super::View;

pub struct LogView {
}
impl LogView {
    pub(crate) fn new() -> LogView {
        LogView {
        }
    }
}

impl View for LogView {
    fn mapped_events(&self, _app: &App) -> Vec<StravaEvent> {
        vec![
            StravaEvent::ToggleLogView,
            StravaEvent::Up,
            StravaEvent::Down,
            StravaEvent::Quit,
        ]
    }
    fn handle(&mut self, app: &mut crate::app::App, key: crate::event::keymap::MappedKey) {
        match key.strava_event {
            StravaEvent::Quit => app.quit = true,
            StravaEvent::ToggleLogView => app.switch_to_previous(),
            StravaEvent::Down => app.log_view_state.transition(&tui_logger::TuiWidgetEvent::NextPageKey),
            StravaEvent::Up => app.log_view_state.transition(&tui_logger::TuiWidgetEvent::PrevPageKey),
            StravaEvent::Enter => app.log_view_state.transition(&tui_logger::TuiWidgetEvent::EscapeKey),
            _ => (),
        }
    }

    fn draw(
        &mut self,
        app: &mut crate::app::App,
        f: &mut tui::prelude::Buffer,
        area: tui::layout::Rect,
    ) {
        let tui_w = TuiLoggerWidget::default()
            .output_separator('|')
            .output_timestamp(Some("%F %H:%M:%S%.3f".to_string()))
            .output_level(Some(TuiLoggerLevelOutput::Long))
            .output_target(false)
            .style_error(Style::default().fg(Color::Red))
            .style_debug(Style::default().fg(Color::Green))
            .style_warn(Style::default().fg(Color::Yellow))
            .style_trace(Style::default().fg(Color::Magenta))
            .style_info(Style::default().fg(Color::Cyan))
            .output_file(false)
            .output_line(false)
            .state(&mut app.log_view_state)
            .style(Style::default().fg(Color::White).bg(Color::Black));
        tui_w.render(area, f);
    }
}
