use crossterm::event::KeyEvent;


pub fn map_key(ke: KeyEvent) -> MappedKey {
    match ke {
        _ => new_strava_key(ke, StravaEvent::None)
    }
}

fn new_strava_key(ke: KeyEvent, se: StravaEvent) -> MappedKey {
    MappedKey { key_event: ke, strava_event: se }
}


pub struct MappedKey {
    key_event: KeyEvent,
    strava_event: StravaEvent,
}

pub enum StravaEvent {
    ToggleUnitSystem,
    Down,
    Up,
    Enter,
    None,
}

