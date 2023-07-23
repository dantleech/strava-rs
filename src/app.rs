use std::{
    cmp::Ordering,
    fmt::Display,
    io,
    time::{Duration, SystemTime},
};

use strum::EnumIter;

use tokio::{select, sync::mpsc::Receiver};
use tui::{
    backend::{Backend, CrosstermBackend},
    widgets::TableState,
    Frame, Terminal,
};
use tui_textarea::TextArea;

use crate::{
    component::activity_list::ActivityListState, input::InputEvent, store::activity::ActivityStore,
};
use crate::{
    component::{activity_list, activity_view, unit_formatter::UnitFormatter},
    event::keymap::{map_key, MappedKey},
    store::activity::{Activity, ActivitySplit},
    ui,
};

pub struct ActivityFilters {
    pub sort_by: SortBy,
    pub sort_order: SortOrder,
    pub filter: String,
}

pub struct Notification {
    text: String,
    created: SystemTime,
}

impl Notification {
    pub fn new(text: String) -> Self {
        Self {
            text,
            created: SystemTime::now(),
        }
    }

    fn has_expired(&self) -> bool {
        self.created.elapsed().unwrap() > Duration::from_secs(5)
    }
}
impl Display for Notification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

pub struct App<'a> {
    pub quit: bool,
    pub active_page: ActivePage,
    pub unit_formatter: UnitFormatter,
    pub activity_list: ActivityListState<'a>,
    pub filters: ActivityFilters,

    pub activity_type: Option<String>,
    pub activity: Option<Activity>,
    pub activities: Vec<Activity>,

    pub info_message: Option<Notification>,
    pub error_message: Option<Notification>,

    store: &'a mut ActivityStore<'a>,
    event_receiver: Receiver<InputEvent>,
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
    pub fn new<'a>(
        store: &'a mut ActivityStore<'a>,
        event_receiver: Receiver<InputEvent>,
    ) -> App<'a> {
        App {
            quit: false,
            active_page: ActivePage::ActivityList,
            unit_formatter: UnitFormatter::imperial(),
            activity_list: ActivityListState {
                table_state: TableState::default(),
                filter_text_area: TextArea::default(),
                filter_dialog: false,
                sort_dialog: false,
            },
            filters: ActivityFilters {
                sort_by: SortBy::Date,
                sort_order: SortOrder::Desc,
                filter: "".to_string(),
            },
            activity: None,
            activities: store.activities(),
            store,

            activity_type: None,
            info_message: None,
            error_message: None,
            event_receiver,
        }
    }
    pub async fn run(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    ) -> Result<(), anyhow::Error> {
        loop {
            if self.quit {
                break;
            }
            terminal.draw(|f| {
                self.draw(f).expect("Could not draw frame");
            })?;

            if let Some(message) = &self.info_message {
                if message.has_expired() {
                    self.info_message = None
                }
            }
            if let Some(message) = &self.error_message {
                if message.has_expired() {
                    self.error_message = None
                }
            }

            if let Some(event) = self.event_receiver.recv().await {
                match event {
                    InputEvent::Input(k) => {
                        let key = map_key(k);
                        self.handle(key);
                    }
                    InputEvent::InfoMessage(message) => {
                        self.info_message = Some(Notification::new(message));
                    }
                    InputEvent::ErrorMessage(message) => {
                        self.error_message = Some(Notification::new(message));
                    }
                    InputEvent::Tick => (),
                }
            }
        }
        Ok(())
    }

    // TODO: Add a collection object
    pub fn unsorted_filtered_activities(&self) -> Vec<Activity> {
        let activities = self.activities.clone();
        activities
            .into_iter()
            .filter(|a| {
                if !a.title.contains(self.filters.filter.as_str()) {
                    return false;
                }
                if let Some(activity_type) = self.activity_type.clone() {
                    if a.activity_type != activity_type {
                        return false;
                    }
                }

                true
            })
            .collect()
    }

    pub fn filtered_activities(&self) -> Vec<Activity> {
        let mut activities = self.unsorted_filtered_activities();
        activities.sort_by(|a, b| {
            let ordering = match self.filters.sort_by {
                SortBy::Date => a.id.cmp(&b.id),
                SortBy::Distance => a
                    .distance
                    .partial_cmp(&b.distance)
                    .unwrap_or(Ordering::Less),
                SortBy::Pace => a.kmph().partial_cmp(&b.kmph()).unwrap_or(Ordering::Less),
                SortBy::HeartRate => a
                    .average_heartrate
                    .or(Some(0.0))
                    .partial_cmp(&b.average_heartrate.or(Some(0.0)))
                    .unwrap(),
                SortBy::Time => a.moving_time.partial_cmp(&b.moving_time).unwrap(),
            };
            match self.filters.sort_order {
                SortOrder::Asc => ordering,
                SortOrder::Desc => ordering.reverse(),
            }
        });
        activities
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

    pub(crate) fn activity_splits(&mut self, activity: Activity) -> Vec<ActivitySplit> {
        self.store.splits(activity)
    }

    fn tick(&self) {}
}
