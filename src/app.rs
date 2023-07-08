use std::{cmp::Ordering, fmt::Display, io, time::Duration};
use strum::EnumIter;

use crossterm::event::{self, poll, Event};
use tui::{
    backend::{Backend, CrosstermBackend},
    widgets::TableState,
    Frame, Terminal,
};
use tui_textarea::TextArea;

use crate::{
    component::{activity_list, activity_view, unit_formatter::UnitFormatter},
    event::keymap::{map_key, MappedKey},
    store::activity::Activity,
    ui,
};

pub struct App<'a> {
    pub quit: bool,
    pub active_page: ActivePage,

    pub unit_formatter: UnitFormatter,
    pub activity_list_table_state: TableState,
    pub activity_list_filter_text_area: TextArea<'a>,
    pub activity_list_filter_dialog: bool,
    pub activity_list_sort_dialog: bool,
    pub activity_list_sort_by: SortBy,
    pub activity_list_sort_order: SortOrder,
    pub activity_list_filter: String,
    pub activity: Option<Activity>,
    pub activities: Vec<Activity>,
}

pub enum ActivePage {
    ActivityList,
    Activity,
}

#[derive(EnumIter)]
pub enum SortBy {
    Date,
    Distance,
    Pace,
    HeartRate,
    Time,
}

impl Display for SortBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_label())
    }
}

pub enum SortOrder {
    Asc,
    Desc,
}

impl Display for SortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SortOrder::Asc => "ascending",
                SortOrder::Desc => "descending",
            }
        )
    }
}

impl App<'_> {
    pub fn new() -> App<'static> {
        App {
            quit: false,
            active_page: ActivePage::ActivityList,
            unit_formatter: UnitFormatter::imperial(),

            activities: vec![],
            activity: None,

            activity_list_table_state: TableState::default(),
            activity_list_filter_text_area: TextArea::default(),
            activity_list_filter_dialog: false,
            activity_list_filter: "".to_string(),
            activity_list_sort_dialog: false,
            activity_list_sort_by: SortBy::Date,
            activity_list_sort_order: SortOrder::Desc,
        }
    }
    pub fn run<'a>(
        &mut self,
        terminal: &'a mut Terminal<CrosstermBackend<io::Stdout>>,
    ) -> Result<(), anyhow::Error> {
        loop {
            if self.quit {
                break;
            }
            terminal.draw(|f| {
                self.draw(f).expect("Could not draw frame");
            })?;
            if (poll(Duration::from_millis(1000)))? {
                if let Event::Key(key) = event::read()? {
                    let key = map_key(key);
                    self.handle(key)
                }
            }
        }
        Ok(())
    }

    pub fn filtered_activities(&self) -> Vec<Activity> {
        let mut activities = self.activities.clone();
        activities.sort_by(|a, b| {
            let ordering = match self.activity_list_sort_by {
                SortBy::Date => a.id.cmp(&b.id),
                SortBy::Distance => a
                    .distance
                    .partial_cmp(&b.distance)
                    .or(Some(Ordering::Less))
                    .unwrap(),
                SortBy::Pace => a
                    .kmph()
                    .partial_cmp(&b.kmph())
                    .or(Some(Ordering::Less))
                    .unwrap(),
                SortBy::HeartRate => a
                    .average_heartrate
                    .or(Some(0.0))
                    .partial_cmp(&b.average_heartrate.or(Some(0.0)))
                    .unwrap(),
                SortBy::Time => a
                    .moving_time
                    .partial_cmp(&b.moving_time)
                    .unwrap(),
            };
            match self.activity_list_sort_order {
                SortOrder::Asc => ordering,
                SortOrder::Desc => ordering.reverse(),
            }
        });
        activities
            .into_iter()
            .filter(|a| a.title.contains(self.activity_list_filter.as_str()))
            .collect()
    }

    fn draw<B: Backend>(&mut self, f: &mut Frame<B>) -> Result<(), anyhow::Error> {
        ui::draw(self, f)
    }

    fn handle(&mut self, key: MappedKey) {
        match self.active_page {
            ActivePage::ActivityList => activity_list::handle(self, key),
            ActivePage::Activity => activity_view::handle(self, key),
        }
    }
}
