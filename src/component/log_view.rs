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
    state: TuiWidgetState,
}
impl LogView {
    pub(crate) fn new() -> LogView {
        LogView {
            state: TuiWidgetState::default(),
        }
    }
}

impl View for LogView {
    fn mapped_events(&self, _app: &App) -> Vec<StravaEvent> {
        vec![StravaEvent::ToggleLogView, StravaEvent::Quit]
    }
    fn handle(&mut self, app: &mut crate::app::App, key: crate::event::keymap::MappedKey) {
        info!("Log view");
        match key.strava_event {
            StravaEvent::Quit => app.quit = true,
            StravaEvent::ToggleLogView => app.switch_to_previous(),
            StravaEvent::Up => self.state.transition(&tui_logger::TuiWidgetEvent::UpKey),
            StravaEvent::Down => self.state.transition(&tui_logger::TuiWidgetEvent::DownKey),
            StravaEvent::Filter => self.state.transition(&tui_logger::TuiWidgetEvent::HideKey),
            StravaEvent::Next => self.state.transition(&tui_logger::TuiWidgetEvent::NextPageKey),
            StravaEvent::Previous => self.state.transition(&tui_logger::TuiWidgetEvent::PrevPageKey),
            _ => (),
        }
    }

    fn draw(
        &mut self,
        _app: &mut crate::app::App,
        f: &mut tui::prelude::Buffer,
        area: tui::layout::Rect,
    ) {
        let tui_w = TuiLoggerSmartWidget::default()
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
            .state(&mut self.state)
            .style(Style::default().fg(Color::White).bg(Color::Black));
        tui_w.render(area, f);
    }
}
