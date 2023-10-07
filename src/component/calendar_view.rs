use chrono::{Datelike, NaiveDate};
use time::{Date, Month};
use tui::{
    prelude::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Line},
    widgets::{
        calendar::{CalendarEventStore, Monthly},
        Axis, BarChart, BarGroup, Chart, Dataset, GraphType, Widget, Bar,
    },
};

use crate::{
    app::App,
    event::keymap::StravaEvent,
    store::activity::{Activities, Activity},
};

use super::{activity_list::list::activity_list_table, View, unit_formatter::UnitFormatter};

pub struct CalendarView {}
impl CalendarView {
    pub(crate) fn new() -> CalendarView {
        CalendarView {}
    }

    fn calendar_widget(&self, app: &mut App) -> Monthly<CalendarEventStore> {
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

        Monthly::new(
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
        .show_surrounding(Style::default().fg(Color::DarkGray))
    }

    fn month_widget<'a>(
        &self,
        data: Vec<(u8, u64)>,
        unit_formatter: &UnitFormatter,
        selected_date: NaiveDate,
    ) -> BarChart<'a> {
        let group = BarGroup::default().bars(
            &data
                .iter()
                .map(|(day, distance)| {
                    Bar::default()
                        .value(*distance as u64)
                        .text_value(unit_formatter.distance(*distance as f64))
                        .label(Line::from(day.to_string()))
                        .style({
                            match day == &(selected_date.day() as u8) {
                                true => Style::default().fg(Color::Red),
                                false => Style::default().fg(Color::Green),
                            }
                        })
                }).collect::<Vec<Bar>>(),
        );
        BarChart::default()
            .bar_style(Style::default().fg(Color::Green))
            .value_style(Style::default().fg(Color::White))
            .bar_width(7)
            .data(group)
    }

    fn month_data(&self, app: &mut App) -> Vec<(u8, u64)> {
        let dim = time::util::days_in_year_month(
            app.calendar_view_state.year,
            app.calendar_view_state.month,
        );
        let mut d = 1;
        let mut data = vec![];
        while d <= dim {
            let activities = app.activities.for_date(
                NaiveDate::from_ymd_opt(
                    app.calendar_view_state.year,
                    app.calendar_view_state.month as u32,
                    d as u32,
                )
                .unwrap(),
            );
            data.push((d, activities.distance() as u64));
            d += 1;
        }

        data
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
        self.day = 1;
    }

    fn previous_month(&mut self) {
        self.month = self.month.previous();
        self.day = 31;
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

    fn selected_date(&self) -> chrono::NaiveDate {
        return NaiveDate::from_ymd_opt(self.year, self.month as u32, self.day as u32)
            .expect("Could not convert date");
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
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(8), Constraint::Length(2)].as_ref())
            .split(area);
        let cal_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(30), Constraint::Length(10)].as_ref())
            .split(rows[0]);

        let cal_w = self.calendar_widget(app);
        cal_w.render(cal_row[0], f);

        let a = app
            .activities()
            .for_date(app.calendar_view_state.selected_date());
        activity_list_table(app, &a).render(cal_row[1], f);

        let data = self.month_data(app);
        self.month_widget(
            data,
            &app.unit_formatter,
            app.calendar_view_state.selected_date()
        ).render(rows[1], f);
    }
}
