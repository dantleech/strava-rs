use crossterm::event::{KeyEvent, KeyCode};

pub fn map_key(ke: KeyEvent) -> MappedKey {
    match ke.code {
        KeyCode::Char('q') => new_strava_key(ke, StravaEvent::Quit),
        KeyCode::Char('k') => new_strava_key(ke, StravaEvent::Up),
        KeyCode::Char('j') => new_strava_key(ke, StravaEvent::Down),
        _ => new_strava_key(ke, StravaEvent::None),
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

pub enum StravaEvent {
    ToggleUnitSystem,
    Down,
    Up,
    Enter,
    None,
    Quit,
}