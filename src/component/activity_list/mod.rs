pub mod chart;
pub mod list;
pub mod rank_dialog;
pub mod sort_dialog;

use crossterm::event::Event;
use tui::{
    layout::{Constraint, Layout},
    prelude::Buffer,
    style::Style,
    widgets::{Block, Borders, Clear, Paragraph, StatefulWidget, Table, TableState, Widget},
};
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;

use crate::{
    app::App,
    event::{
        input::InputEvent,
        keymap::{MappedKey, StravaEvent},
    },
    store::activity::SortOrder,
    ui::{centered_rect_absolute, color::ColorTheme},
};

use self::list::activity_list_table;

use super::{table_status_select_current, View};

pub struct ActivityList {
    cursor_pos: Option<(u16,u16)>
}
impl ActivityList {
    pub(crate) fn new() -> ActivityList {
        ActivityList{
            cursor_pos: None,
        }
    }
}

impl View for ActivityList {
    fn cursor_position(&self) -> Option<(u16,u16)> {
        self.cursor_pos
    }
    fn handle(&mut self, app: &mut App, key: MappedKey) {
        if app.activity_list.filter_dialog {
            let matched = match key.strava_event {
                StravaEvent::Enter => {
                    app.filters.filter = app.activity_list.filter_text_area.value().to_string();
                    app.activity_list.filter_dialog = false;
                    app.activity_list.table_state().select(Some(0));
                    app.send(InputEvent::Reload);
                    true
                }
                _ => false,
            };
            if matched {
                return;
            }

            app.activity_list
                .filter_text_area
                .handle_event(&Event::Key(key.key_event));
            return;
        }

        if app.activity_list.sort_dialog {
            sort_dialog::handle(app, key);

            return;
        }
        if app.activity_list.rank_dialog {
            rank_dialog::handle(app, key);

            return;
        }
        match key.strava_event {
            StravaEvent::Quit => app.quit = true,
            StravaEvent::ToggleUnitSystem => {
                app.unit_formatter = app.unit_formatter.toggle();
            }
            StravaEvent::ToggleSortOrder => {
                app.filters.sort_order = match app.filters.sort_order {
                    SortOrder::Asc => SortOrder::Desc,
                    SortOrder::Desc => SortOrder::Asc,
                };
                app.send(InputEvent::Reload);
            }
            StravaEvent::Down => app.next_activity(),
            StravaEvent::Up => app.previous_activity(),
            StravaEvent::Filter => toggle_filter(app),
            StravaEvent::Sort => toggle_sort(app),
            StravaEvent::Rank => toggle_rank(app),
            StravaEvent::Enter => table_status_select_current(app),
            StravaEvent::Refresh => app.send(InputEvent::Sync),
            StravaEvent::IncreaseTolerance => {
                app.filters.anchor_tolerance_add(0.01);
                app.send(InputEvent::Reload)
            }
            StravaEvent::DecreaseTolerance => {
                app.filters.anchor_tolerance_add(-0.01);
                app.send(InputEvent::Reload);
            }
            StravaEvent::Anchor => {
                app.anchor_selected();
                app.send(InputEvent::Reload);
            }
            _ => (),
        }
    }

    fn mapped_events(&self, app: &App) -> Vec<StravaEvent> {
        let mut events = vec![
            StravaEvent::Down,
            StravaEvent::Up,
            StravaEvent::ToggleUnitSystem,
            StravaEvent::Filter,
            StravaEvent::Sort,
            StravaEvent::Rank,
            StravaEvent::Refresh,
        ];
        events.push(StravaEvent::Anchor);
        if app.activity_anchored.is_some() {
            events.push(StravaEvent::IncreaseTolerance);
            events.push(StravaEvent::DecreaseTolerance);
        }
        events.push(StravaEvent::Quit);
        return events;
    }

    fn draw(&mut self, app: &mut App, f: &mut Buffer, area: tui::layout::Rect) {
        let rows = Layout::default()
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);
        chart::draw(app, f, rows[1]);

        let activities = &app.activities();

        if app.activity_list.table_state().selected().is_none() && !activities.is_empty() {
            app.activity_list.table_state().select(Some(0));
        }

        let table = activity_list_table(app, activities);
        <Table as StatefulWidget>::render(table, rows[0], f, app.activity_list.table_state());

        if app.activity_list.filter_dialog {
            let rect = centered_rect_absolute(64, 3, area);
            let p = Paragraph::new(app.activity_list.filter_text_area.value()).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Filter")
                    .border_style(Style::default().fg(ColorTheme::Dialog.to_color())),
            );

            self.cursor_pos = Some((
                1 + rect.x + app.activity_list.filter_text_area.visual_cursor() as u16,
                rect.y + 1,
            ));

            Clear.render(rect, f);
            p.render(rect, f);

            return;
        }

        if app.activity_list.sort_dialog {
            sort_dialog::draw(app, f, area);

            return;
        }
        if app.activity_list.rank_dialog {
            rank_dialog::draw(app, f, area);
        }
    }
}

pub enum ActivityListMode {
    Normal,
    Anchored,
}

pub struct ActivityListState {
    pub mode: ActivityListMode,
    pub table_state: TableState,
    pub anchored_table_state: TableState,
    pub filter_text_area: Input,
    pub filter_dialog: bool,
    pub sort_dialog: bool,
    pub rank_dialog: bool,
}

pub struct ActivityViewState {
    pub pace_table_state: TableState,
    pub selected_split: Option<i64>,
}
impl ActivityViewState {
    pub(crate) fn select_split(&mut self, selected: i64) {
        self.selected_split = Some(selected);
    }
}

impl ActivityListState {
    pub fn table_state(&mut self) -> &mut TableState {
        match self.mode {
            ActivityListMode::Normal => &mut self.table_state,
            ActivityListMode::Anchored => &mut self.anchored_table_state,
        }
    }
}
fn toggle_filter(app: &mut App) {
    app.activity_list.filter_dialog = !app.activity_list.filter_dialog;
}

fn toggle_sort(app: &mut App) {
    app.activity_list.sort_dialog = !app.activity_list.sort_dialog;
}
fn toggle_rank(app: &mut App) {
    app.activity_list.rank_dialog = !app.activity_list.rank_dialog;
}
