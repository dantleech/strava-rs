use crossterm::event::{KeyCode, KeyEvent};

pub fn map_key(ke: KeyEvent) -> MappedKey {
    match ke.code {
        KeyCode::Char('q') => new_strava_key(ke, StravaEvent::Quit),
        KeyCode::Char('k') => new_strava_key(ke, StravaEvent::Up),
        KeyCode::Char('j') => new_strava_key(ke, StravaEvent::Down),
        KeyCode::Char('o') => new_strava_key(ke, StravaEvent::ToggleSortOrder),
        KeyCode::Char('u') => new_strava_key(ke, StravaEvent::ToggleUnitSystem),
        KeyCode::Char('s') => new_strava_key(ke, StravaEvent::Sort),
        KeyCode::Char('f') => new_strava_key(ke, StravaEvent::Filter),
        KeyCode::Char('r') => new_strava_key(ke, StravaEvent::Refresh),
        KeyCode::Char('t') => new_strava_key(ke, StravaEvent::AlternateView),
        KeyCode::Enter => new_strava_key(ke, StravaEvent::Enter),
        KeyCode::Esc => new_strava_key(ke, StravaEvent::Escape),
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
    AlternateView,
    ToggleUnitSystem,
    ToggleSortOrder,
    Refresh,
    Filter,
    Sort,
    Down,
    Up,
    Enter,
    Escape,
    None,
    Quit,
}
