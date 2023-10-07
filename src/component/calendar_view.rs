use chrono::Datelike;
use time::{Date, Month};
use tui::{
    style::{Color, Style},
    widgets::{
        calendar::{CalendarEventStore, Monthly},
        Widget,
    },
};

use crate::{app::App, event::keymap::StravaEvent};

use super::View;

pub struct CalendarView {}
impl CalendarView {
    pub(crate) fn new() -> CalendarView {
        CalendarView {}
    }
}

pub struct CalendarViewState {
    day: u8,
    month: Month,
    year: i32,
}
impl CalendarViewState {
    fn next_month(&mut self) {
        self.month = self.month.next();
        self.santize_day();
    }

    fn previous_month(&mut self) {
        self.month = self.month.previous();
        self.santize_day();
    }

    fn previous_day(&mut self) {
        if self.day - 1 < 1 {
            self.previous_month();
            self.santize_day();
            return;
        }
        self.day = self.day - 1
    }

    fn next_day(&mut self) {
        if self.day + 1 > time::util::days_in_year_month(self.year, self.month) {
            self.next_month();
            self.santize_day();
            return;
        }
        self.day = self.day + 1
    }

    pub(crate) fn new() -> CalendarViewState {
        CalendarViewState {
            day: 1,
            month: Month::September,
            year: 2023,
        }
    }

    fn santize_day(&mut self) {
        let dim = time::util::days_in_year_month(self.year, self.month);
        if self.day > dim {
            self.day = dim;
        }
        if self.day < 1 {
            self.day = 1
        }
    }

    fn is_selected_equal_to(&self, start_date: Option<chrono::NaiveDateTime>) -> bool {
        start_date.unwrap().year() == self.year
            && Month::try_from(start_date.unwrap().month() as u8).unwrap() == self.month
            && start_date.unwrap().day() as u8 == self.day
    }
}

impl View for CalendarView {
    fn mapped_events(&self, _app: &App) -> Vec<StravaEvent> {
        vec![StravaEvent::Quit, StravaEvent::ActivityListView]
    }
    fn handle(&mut self, app: &mut crate::app::App, key: crate::event::keymap::MappedKey) {
        match key.strava_event {
            StravaEvent::ActivityListView => app.switch_to(crate::app::ActivePage::ActivityList),
            StravaEvent::Next => app.calendar_view_state.next_month(),
            StravaEvent::Previous => app.calendar_view_state.previous_month(),
            StravaEvent::Up => app.calendar_view_state.previous_day(),
            StravaEvent::Down => app.calendar_view_state.next_day(),
            StravaEvent::Quit => app.quit = true,
            _ => (),
        }
    }

    fn draw(
        &mut self,
        app: &mut crate::app::App,
        f: &mut tui::prelude::Buffer,
        area: tui::layout::Rect,
    ) {
        let mut events = CalendarEventStore::default();
        events.add(
            Date::from_calendar_date(
                app.calendar_view_state.year,
                app.calendar_view_state.month,
                app.calendar_view_state.day,
            )
            .unwrap(),
            Style::default().fg(Color::Blue),
        );

        for activity in app.activities() {
            let mut style = Style::default().bg(Color::Blue);
            if app
                .calendar_view_state
                .is_selected_equal_to(activity.start_date)
            {
                style = style.fg(Color::Black);
            }

            events.add(
                Date::from_calendar_date(
                    activity.start_date.unwrap().year(),
                    Month::try_from(activity.start_date.unwrap().month() as u8).unwrap(),
                    activity.start_date.unwrap().day() as u8,
                )
                .unwrap(),
                style,
            );
        }

        let tui_w = Monthly::new(
            Date::from_calendar_date(
                app.calendar_view_state.year,
                app.calendar_view_state.month,
                app.calendar_view_state.day,
            )
            .unwrap(),
            events,
        )
        .show_month_header(Style::default())
        .show_weekdays_header(Style::default())
        .show_surrounding(Style::default().fg(Color::DarkGray));

        tui_w.render(area, f);
    }
}
