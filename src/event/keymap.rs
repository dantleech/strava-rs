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
        map.insert(KeyCode::Char('o'), StravaEvent::ToggleSortOrder);
        map.insert(KeyCode::Char('u'), StravaEvent::ToggleUnitSystem);
        map.insert(KeyCode::Char('s'), StravaEvent::Sort);
        map.insert(KeyCode::Char('S'), StravaEvent::Rank);
        map.insert(KeyCode::Char('f'), StravaEvent::Filter);
        map.insert(KeyCode::Char('r'), StravaEvent::Refresh);
        map.insert(KeyCode::Char('a'), StravaEvent::Anchor);
        map.insert(KeyCode::Char('+'), StravaEvent::IncreaseTolerance);
        map.insert(KeyCode::Char('-'), StravaEvent::DecreaseTolerance);
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

#[derive(Clone)]
pub enum StravaEvent {
    Rank,
    ToggleUnitSystem,
    ToggleSortOrder,
    Refresh,
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
}
