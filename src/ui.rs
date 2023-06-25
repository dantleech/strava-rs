use crossterm::event::{KeyEvent, KeyModifiers, KeyCode};
use tui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    widgets::Paragraph,
    Frame,
};
use tui_textarea::{Input, Key};

use crate::{app::{App, ActivePage}, component::{activity_list, activity_view}};

pub fn draw<B: Backend>(app: &mut App, f: &mut Frame<B>) -> Result<(), anyhow::Error> {
    let rows = Layout::default()
        .margin(0)
        .constraints([Constraint::Min(4)].as_ref())
        .split(f.size());

    match app.active_page {
        ActivePage::ActivityList => {
            activity_list::draw(app, f, rows[0])?;
        }
        ActivePage::Activity => {
            activity_view::draw(app, f, rows[0])?;
        }
    }

    Ok(())
}

// borrowed from https://github.com/extrawurst/gitui
pub fn centered_rect_absolute(width: u16, height: u16, r: Rect) -> Rect {
    Rect::new(
        (r.width.saturating_sub(width)) / 2,
        (r.height.saturating_sub(height)) / 2,
        width.min(r.width),
        height.min(r.height),
    )
}

// because we need crossterm 0.26: https://github.com/rhysd/tui-textarea/issues/9
pub fn key_event_to_input(key: KeyEvent) -> Input {
    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
    let alt = key.modifiers.contains(KeyModifiers::ALT);
    let key = match key.code {
        KeyCode::Char(c) => Key::Char(c),
        KeyCode::Backspace => Key::Backspace,
        KeyCode::Enter => Key::Enter,
        KeyCode::Left => Key::Left,
        KeyCode::Right => Key::Right,
        KeyCode::Up => Key::Up,
        KeyCode::Down => Key::Down,
        KeyCode::Tab => Key::Tab,
        KeyCode::Delete => Key::Delete,
        KeyCode::Home => Key::Home,
        KeyCode::End => Key::End,
        KeyCode::PageUp => Key::PageUp,
        KeyCode::PageDown => Key::PageDown,
        KeyCode::Esc => Key::Esc,
        KeyCode::F(x) => Key::F(x),
        _ => Key::Null,
    };
    Input { key, ctrl, alt }
}
