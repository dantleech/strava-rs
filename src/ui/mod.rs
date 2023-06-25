pub mod color;

use crossterm::event::{KeyEvent, KeyModifiers, KeyCode};

use tui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    Frame, widgets::{Paragraph, Block, BorderType, Borders}, text::{Span, Spans}, style::{Style, Color, Modifier},
};
use tui_textarea::{Input, Key};

use crate::{app::{App, ActivePage}, component::{activity_list, activity_view}};

use self::color::ColorTheme;

pub fn draw<B: Backend>(app: &mut App, f: &mut Frame<B>) -> Result<(), anyhow::Error> {
    let rows = Layout::default()
        .margin(0)
        .constraints([Constraint::Length(3), Constraint::Min(4), Constraint::Length(1)].as_ref())
        .split(f.size());

    f.render_widget(header(app), rows[0]);

    match app.active_page {
        ActivePage::ActivityList => {
            activity_list::draw(app, f, rows[1])?;
        }
        ActivePage::Activity => {
            activity_view::draw(app, f, rows[1])?;
        }
    }

    f.render_widget(status_bar(app), rows[2]);

    Ok(())
}

fn header<'a>(_app: &'a mut App) -> Paragraph<'a> {
    let strava = ColorTheme::Orange.to_color();
    let text: Vec<Spans> = vec![Spans::from(vec![

        Span::styled("[n]", Style::default().fg(strava)),
        Span::raw("ext "),
        Span::styled("[p]", Style::default().fg(strava)),
        Span::raw("rev "),
        Span::styled("[u]", Style::default().fg(strava)),
        Span::raw("nit toggle "),
        Span::styled("[f]", Style::default().fg(strava)),
        Span::raw("ilter "),
        Span::styled("[q]", Style::default().fg(strava)),
        Span::raw("uit"),
    ])];

    Paragraph::new(text).block(Block::default().borders(Borders::ALL).style(Style::default()))
}

fn status_bar<'a>(app: &'a mut App) -> Paragraph<'a> {
    let mut status: Vec<String> = Vec::new();
    if app.activity_list_filter != "".to_string() {
        status.push(format!("filtered by \"{}\"", app.activity_list_filter))
    }
    status.push(format!("{} activities", app.filtered_activities().len()));
    status.push(format!("{} units", app.unit_formatter.system.to_string()));

    Paragraph::new(status.join(", "))
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
