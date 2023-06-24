use std::{io, time::Duration};

use crossterm::event::{self, poll, Event};
use tui::{
    backend::{Backend, CrosstermBackend},
    widgets::TableState,
    Frame, Terminal,
};

use crate::{
    component::{activity_list, unit_formatter::UnitFormatter},
    event::keymap::{map_key, MappedKey},
    store::activity::Activity,
};

pub struct App {
    pub quit: bool,
    pub active_page: ActivePage,

    pub unit_formatter: UnitFormatter,
    pub activity_list_table_state: TableState,
    pub activities: Vec<Activity>,
}

pub enum ActivePage {
    ActivityList,
}

impl App {
    pub fn new() -> App {
        App {
            quit: false,
            active_page: ActivePage::ActivityList,
            unit_formatter: UnitFormatter::imperial(),

            activities: vec![],

            activity_list_table_state: TableState::default(),
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

    fn draw<B: Backend>(&mut self, f: &mut Frame<B>) -> Result<(), anyhow::Error> {
        Ok(())
    }

    fn handle(&mut self, key: MappedKey) {
        match self.active_page {
            ActivePage::ActivityList => activity_list::handle(self, key),
        }
    }
}
