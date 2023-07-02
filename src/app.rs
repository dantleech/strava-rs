use std::{io, time::Duration};

use crossterm::event::{self, poll, Event};
use tui::{
    backend::{Backend, CrosstermBackend},
    widgets::TableState,
    Frame, Terminal,
};
use tui_textarea::TextArea;

use crate::{
    component::{activity_list, unit_formatter::UnitFormatter, activity_view},
    event::keymap::{map_key, MappedKey},
    store::activity::Activity, ui,
};

pub struct App<'a> {
    pub quit: bool,
    pub active_page: ActivePage,

    pub unit_formatter: UnitFormatter,
    pub activity_list_table_state: TableState,
    pub activity_list_filter_text_area: TextArea<'a>,
    pub activity_list_filter_dialog: bool,
    pub activity_list_sort_dialog: bool,
    pub activity_list_filter: String,
    pub activity: Option<Activity>,
    pub activities: Vec<Activity>,
}

pub enum ActivePage {
    ActivityList,
    Activity,
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
        self.activities.clone().into_iter().filter(|a|a.title.contains(self.activity_list_filter.as_str())).collect()
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
