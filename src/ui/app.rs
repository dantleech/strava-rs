use std::{io, time::Duration};

use crossterm::event::{self, poll, Event};
use tui::{
    backend::{Backend, CrosstermBackend},
    Frame, Terminal,
};

use super::{layout::AppLayout};

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
                        event::KeyCode::Char('u') => self.handle(super::event::StravaEvent::ToggleUnitSystem),
                        event::KeyCode::Char('j') => self.handle(super::event::StravaEvent::Down),
                        event::KeyCode::Char('k') => self.handle(super::event::StravaEvent::Up),
                        _ => (),
                    }
                }
            }
        }
        Ok(())
    }

    fn draw<B: Backend>(&mut self, f: &mut Frame<B>) -> Result<(), anyhow::Error> {
        self.layout.draw(f)
    }

    fn handle(&mut self, event: super::event::StravaEvent) {
        self.layout.handle(event)
    }
}
