use std::{
    fmt::Display,
    io,
    time::{Duration, SystemTime},
};

use tokio::sync::mpsc::{Receiver, Sender};
use tui::{
    backend::{Backend, CrosstermBackend},
    widgets::TableState, Terminal,
};
use tui_input::Input;

use crate::{
    component::{activity_list, unit_formatter::UnitFormatter},
    event::keymap::KeyMap,
    store::activity::Activity,
    ui,
};
use crate::{
    component::{
        activity_list::{ActivityList, ActivityListMode, ActivityListState, ActivityViewState},
        activity_view::ActivityView,
        View,
    },
    event::{
        input::EventSender,
        util::{table_state_next, table_state_prev},
    },
    input::InputEvent,
    store::activity::{Activities, ActivityStore, SortBy, SortOrder},
};

pub struct ActivityFilters {
    pub sort_by: SortBy,
    pub sort_order: SortOrder,
    pub anchor_tolerance: f64,
    pub filter: String,
}

pub struct RankOptions {
    pub rank_by: SortBy,
    pub rank_order: SortOrder,
}

impl ActivityFilters {
    pub fn anchor_tolerance_add(&mut self, delta: f64) {
        self.anchor_tolerance += delta;
        if self.anchor_tolerance < 0.0 {
            self.anchor_tolerance = 0.0;
        }
    }
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
    pub activity_list: ActivityListState,
    pub activity_view: ActivityViewState,
    pub filters: ActivityFilters,
    pub ranking: RankOptions,

    pub activity_type: Option<String>,
    pub activity: Option<Activity>,
    pub activity_anchored: Option<Activity>,
    pub activities: Activities,

    pub info_message: Option<Notification>,
    pub error_message: Option<Notification>,
    pub key_map: KeyMap,

    store: &'a mut ActivityStore<'a>,
    event_receiver: Receiver<InputEvent>,
    event_sender: EventSender,

    event_queue: Vec<InputEvent>,
    sync_sender: Sender<bool>,
}

pub enum ActivePage {
    ActivityList,
    Activity,
}

impl App<'_> {
    pub fn new<'a>(
        store: &'a mut ActivityStore<'a>,
        event_receiver: Receiver<InputEvent>,
        event_sender: EventSender,
        sync_sender: Sender<bool>,
    ) -> App<'a> {
        App {
            quit: false,
            active_page: ActivePage::ActivityList,
            unit_formatter: UnitFormatter::imperial(),
            activity_list: ActivityListState {
                mode: activity_list::ActivityListMode::Normal,
                table_state: TableState::default(),
                anchored_table_state: TableState::default(),
                filter_text_area: Input::default(),
                filter_dialog: false,
                sort_dialog: false,
                rank_dialog: false,
            },
            activity_view: ActivityViewState {
                pace_table_state: TableState::default(),
                selected_split: None,
            },
            filters: ActivityFilters {
                sort_by: SortBy::Date,
                sort_order: SortOrder::Desc,
                filter: "".to_string(),
                anchor_tolerance: 0.005,
            },
            ranking: RankOptions {
                rank_by: SortBy::Pace,
                rank_order: SortOrder::Desc,
            },
            activity: None,
            activity_anchored: None,
            activities: Activities::new(),
            store,

            activity_type: None,
            info_message: None,
            error_message: None,
            event_receiver,
            event_sender,
            event_queue: vec![],
            sync_sender,
            key_map: KeyMap::default(),
        }
    }
    pub async fn run(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    ) -> Result<(), anyhow::Error> {
        terminal.clear()?;
        loop {
            if self.quit {
                break;
            }

            let mut view: Box<dyn View> = match self.active_page {
                ActivePage::ActivityList => Box::new(ActivityList::new()),
                ActivePage::Activity => Box::new(ActivityView{}),
            };

            self.render(terminal, view.as_mut())?;

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

            while self.event_queue.len() > 1 {
                self.event_sender
                    .send(self.event_queue.pop().unwrap())
                    .await?;
            }

            if let Some(event) = self.event_receiver.recv().await {
                match event {
                    InputEvent::Input(k) => {
                        let key = self.key_map.map_key(k);
                        view.handle(self, key);
                    }
                    InputEvent::InfoMessage(message) => {
                        self.info_message = Some(Notification::new(message));
                    }
                    InputEvent::ErrorMessage(message) => {
                        self.error_message = Some(Notification::new(message));
                    }
                    InputEvent::Tick => (),
                    InputEvent::Reload => self.reload().await,
                    InputEvent::Sync => self.sync_sender.send(true).await?,
                }
            }
        }
        Ok(())
    }

    pub async fn reload(&mut self) {
        let mut activities = self.store.activities().await;
        activities = activities.where_title_contains(self.filters.filter.as_str());
        if let Some(activity_type) = self.activity_type.clone() {
            activities = activities.having_activity_type(activity_type);
        }
        if let Some(anchored) = &self.activity_anchored {
            activities = activities.withing_distance_of(anchored, self.filters.anchor_tolerance);
        }
        self.activities = activities
            .rank(&self.ranking.rank_by, &self.ranking.rank_order)
            .sort(&self.filters.sort_by, &self.filters.sort_order)
    }

    pub fn activities(&self) -> Activities {
        self.activities.clone()
    }

    pub fn send(&mut self, event: InputEvent) {
        self.event_queue.push(event);
    }

    pub(crate) fn anchor_selected(&mut self) {
        let activities = self.activities();
        if let Some(selected) = self.activity_list.table_state().selected() {
            if let Some(a) = activities.get(selected) {
                if self.activity_anchored.is_some() {
                    self.activity_anchored = None;
                    self.activity_list.mode = ActivityListMode::Normal;
                    return;
                }

                self.activity_anchored = Some(a.clone());
                self.activity_list.mode = ActivityListMode::Anchored;
                self.activity_list.table_state().select(Some(0));
            }
        }
    }

    pub(crate) fn previous_activity(&mut self) {
        table_state_prev(
            self.activity_list.table_state(),
            self.activities.len(),
            false,
        );
        if let Some(selected) = self.activity_list.table_state().selected() {
            if let Some(a) = self.activities.get(selected) {
                self.activity = Some(a.clone());
            }
        }
    }

    pub(crate) fn next_activity(&mut self) {
        table_state_next(
            self.activity_list.table_state(),
            self.activities.len(),
            false,
        );
        if let Some(selected) = self.activity_list.table_state().selected() {
            if let Some(a) = self.activities.get(selected) {
                self.activity = Some(a.clone());
            }
        }
    }

    fn render(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
        view: &mut dyn View
    ) -> Result<(), anyhow::Error> {
        let area = terminal.size().expect("Could not determine terminal size'");
        terminal.autoresize()?;
        let buffer = terminal.current_buffer_mut();
        ui::draw(self, buffer, area, view);

        match view.cursor_position() {
            None => terminal.hide_cursor()?,
            Some((x, y)) => {
                terminal.show_cursor()?;
                terminal.set_cursor(x, y)?;
            }
        }
        terminal.flush()?;
        terminal.swap_buffers();
        terminal.backend_mut().flush()?;
        Ok(())
    }
}
