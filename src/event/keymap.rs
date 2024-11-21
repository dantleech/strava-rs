use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent};

pub struct KeyMap {
    map: HashMap<KeyCode, StravaEvent>
}

impl KeyMap {
    pub fn default() -> KeyMap {
        let mut map = HashMap::new();
        map.insert(KeyCode::Char('q'), StravaEvent::Quit);
        map.insert(KeyCode::Char('k'), StravaEvent::Up);
        map.insert(KeyCode::Char('j'), StravaEvent::Down);
        map.insert(KeyCode::Char('K'), StravaEvent::PageUp);
        map.insert(KeyCode::Char('J'), StravaEvent::PageDown);
        map.insert(KeyCode::Char('n'), StravaEvent::Next);
        map.insert(KeyCode::Char('p'), StravaEvent::Previous);
        map.insert(KeyCode::Char('m'), StravaEvent::MovingElapsed);
        map.insert(KeyCode::Char('o'), StravaEvent::ToggleSortOrder);
        map.insert(KeyCode::Char('u'), StravaEvent::ToggleUnitSystem);
        map.insert(KeyCode::Char('s'), StravaEvent::Sort);
        map.insert(KeyCode::Char('S'), StravaEvent::Rank);
        map.insert(KeyCode::Char('f'), StravaEvent::Filter);
        map.insert(KeyCode::Char('r'), StravaEvent::Refresh);
        map.insert(KeyCode::Char('a'), StravaEvent::Anchor);
        map.insert(KeyCode::Char('+'), StravaEvent::IncreaseTolerance);
        map.insert(KeyCode::Char('-'), StravaEvent::DecreaseTolerance);
        map.insert(KeyCode::Char('0'), StravaEvent::ToggleLogView);
        map.insert(KeyCode::Enter, StravaEvent::Enter);
        map.insert(KeyCode::Esc, StravaEvent::Escape);
        KeyMap{map}
    }

    pub fn map_key(&self, ke: KeyEvent) -> MappedKey {
        match self.map.get(&ke.code) {
            Some(event) => new_strava_key(ke, event.clone()),
            None => new_strava_key(ke, StravaEvent::None),
        }
    }

    pub(crate) fn key(&self, f: &StravaEvent) -> Option<KeyCode> {
        for (key, event) in self.map.clone() {
            if &event == f {
                return Some(key)
            }
        }
        None
    }
}

fn new_strava_key(ke: KeyEvent, se: StravaEvent) -> MappedKey {
    MappedKey {
        key_event: ke,
        strava_event: se,
    }
}

pub struct MappedKey {
    pub key_event: KeyEvent,
    pub strava_event: StravaEvent,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum StravaEvent {
    Rank,
    ToggleUnitSystem,
    ToggleSortOrder,
    Refresh,
    ToggleLogView,
    Filter,
    Anchor,
    Sort,
    Down,
    Up,
    Enter,
    Escape,
    None,
    IncreaseTolerance,
    DecreaseTolerance,
    Quit,
    Next,
    Previous,
    PageDown,
    PageUp,
    MovingElapsed,
}
impl StravaEvent {
    pub fn describe(se: &StravaEvent) -> &'static str {
        match se {
            StravaEvent::Rank => "rank",
            StravaEvent::ToggleUnitSystem => "unit",
            StravaEvent::ToggleSortOrder => "order",
            StravaEvent::Refresh => "refresh",
            StravaEvent::Filter => "filter",
            StravaEvent::Anchor => "anchor",
            StravaEvent::Sort => "sort",
            StravaEvent::Down => "down",
            StravaEvent::Up => "up",
            StravaEvent::Enter => "enter",
            StravaEvent::PageUp => "page up",
            StravaEvent::PageDown => "page down",
            StravaEvent::Escape => "back",
            StravaEvent::None => "none",
            StravaEvent::IncreaseTolerance => "tolerance++",
            StravaEvent::DecreaseTolerance => "tolerance--",
            StravaEvent::Quit => "quit",
            StravaEvent::Next => "next",
            StravaEvent::Previous => "prev",
            StravaEvent::ToggleLogView => "logs",
            StravaEvent::MovingElapsed => "moving/elapsed",
        }
    }
}
