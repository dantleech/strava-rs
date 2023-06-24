use std::{io, time::Duration};

use crossterm::event::{self, poll, Event};
use tui::{
    backend::{Backend, CrosstermBackend},
    Frame, Terminal,
};

use crate::{config::keymap::{map_key, MappedKey}, component::activity_list};

pub struct App {
    quit: bool,
    active_page: ActivePage,
}

enum ActivePage {
    ActivityList
}

impl App {
    pub fn new() -> App {
        App { quit: false }
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

    fn handle(&mut self, _key: MappedKey) {
        match self.active_page {
            ActivePage::ActivityList => activity_list::handle(self),
            _ => panic!("Unkown page")
        }
    }
}
