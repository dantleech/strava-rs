use std::{fmt::Display, f64::INFINITY};

pub struct UnitFormatter {
    pub system: UnitSystem,
}
pub const KILOMETER_TO_MILE: f64 = 0.621371;
const METERS_TO_FOOT: f64 = 3.28084;

pub enum UnitSystem {
    Metric,
    Imperial,
}

impl Display for UnitSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match *self {
            UnitSystem::Metric => "metric",
            UnitSystem::Imperial => "imperial",
        })
    }
}

impl UnitFormatter {
    pub fn stopwatch_time(&self, seconds: i64) -> String {
        if seconds == 0 {
            return "00:00".to_string();
        }
        let hours = seconds / 3600;
        let rem = seconds % 3600;
        let minutes = rem / 60;
        let seconds = rem % 60;
        if hours == 0 {
            return format!("{:0>2}:{:0>2}", minutes, seconds);
        }
        format!("{:0>2}:{:0>2}:{:0>2}", hours, minutes, seconds)
    }

    pub fn distance(&self, quantity: f64) -> String {
        match self.system {
            UnitSystem::Metric => {
                format!("{:.2}km", (quantity / 1000.0))
            }
            UnitSystem::Imperial => {
                format!("{:.2}mi", ((quantity / 1000.0) * KILOMETER_TO_MILE))
            }
        }
    }

    pub fn elevation(&self, elevation: f64) -> String {
        match self.system {
            UnitSystem::Metric => {
                format!("{:.2}m", elevation)
            }
            UnitSystem::Imperial => {
                format!("{:.2}ft", elevation * METERS_TO_FOOT)
            }
        }
    }

    pub fn pace(&self, time: i64, meters: f64) -> String {
        let spm = time as f64 / meters;
        if spm == INFINITY {
            return "N/A".to_string();
        }
        match self.system {
            UnitSystem::Metric => {
                format!("{} /km", self.stopwatch_time((spm * 1000.0).round() as i64))
            }
            UnitSystem::Imperial => {
                format!(
                    "{} /mi",
                    self.stopwatch_time(((spm * 1000.0) / KILOMETER_TO_MILE).round() as i64)
                )
            }
        }
    }

    pub(crate) fn imperial() -> Self {
        UnitFormatter {
            system: UnitSystem::Imperial,
        }
    }
    pub(crate) fn toggle(&self) -> UnitFormatter {
        UnitFormatter {
            system: match self.system {
                UnitSystem::Metric => UnitSystem::Imperial,
                UnitSystem::Imperial => UnitSystem::Metric,
            },
        }
    }

    #[allow(unused)]
    pub(crate) fn speed(&self, meters_per_hour: f64) -> String {
        let kmph = meters_per_hour / 1000.0;
        match self.system {
            UnitSystem::Metric => {
                format!("{:.2}km/h", kmph)
            }
            UnitSystem::Imperial => {
                format!("{:.2}m/h", kmph * 0.621371)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::component::unit_formatter::UnitFormatter;

    #[test]
    fn test_stopwatch_time() {
        let f = UnitFormatter::imperial();
        assert_eq!("00:00", f.stopwatch_time(0));
        assert_eq!("00:30", f.stopwatch_time(30));
        assert_eq!("01:00", f.stopwatch_time(60));
        assert_eq!("01:00:00", f.stopwatch_time(3600));
    }

    #[test]
    fn test_pace() {
        let f = UnitFormatter::imperial();
        assert_eq!("03:13 /mi", f.pace(120, 1000.0));
    }
}
