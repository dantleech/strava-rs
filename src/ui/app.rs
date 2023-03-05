use std::io;

use tui::{Terminal, backend::CrosstermBackend};

pub struct App<'a> {
    terminal: &'a mut Terminal<CrosstermBackend<io::Stdout>>
}
