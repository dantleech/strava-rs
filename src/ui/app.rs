use std::{io, time::Duration};

use crossterm::event::{self, poll, Event};
use tui::{
    backend::{Backend, CrosstermBackend},
    Frame, Terminal,
};

use super::layout::AppLayout;

pub struct App {
    layout: AppLayout,
    quit: bool,
}

impl App {
    pub fn new(layout: AppLayout) -> App {
        App {
            layout,
            quit: false,
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
                    match key.code {
                        event::KeyCode::Char('q') => self.quit = true,
                        _ => (),
                    }
                }
            }
        }
        Ok(())
    }

    fn draw<B: Backend>(&self, f: &mut Frame<B>) -> Result<(), anyhow::Error> {
        self.layout.draw(f)
    }
}
