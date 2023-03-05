use std::{io, time::Duration};

use crossterm::event::{self, poll, Event};
use tui::{
    backend::{Backend, CrosstermBackend},
    Frame, Terminal,
};

pub struct App {}

impl App {
    pub async fn run<'a>(
        &self,
        terminal: &'a mut Terminal<CrosstermBackend<io::Stdout>>,
    ) -> Result<(), anyhow::Error> {
        loop {
            terminal.draw(|f| { self.draw(f).expect("Could not draw frame"); })?;
            if (poll(Duration::from_millis(1000)))? {
                if let Event::Key(_key) = event::read()? {}
            }
        }
    }

    fn draw<B: Backend>(&self, f: &mut Frame<B>) -> Result<(), anyhow::Error> {
        todo!()
    }
}
