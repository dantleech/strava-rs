pub struct UnitFormatter {
    pub system: UnitSystem,
}

pub enum UnitSystem {
    Metric,
    Imperial,
}

impl UnitSystem {
    pub fn to_string(&self) -> String {
        match *self {
            UnitSystem::Metric => String::from("metric"),
            UnitSystem::Imperial => String::from("imperial"),
        }
    }
}

impl UnitFormatter {
    pub fn stopwatch_time(&self, seconds: i32) -> String {
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

    pub fn distance(&self, quantity: f32) -> String {
        match self.system {
            UnitSystem::Metric => {
                format!("{:.2}km", (quantity / 1000.0))
            }
            UnitSystem::Imperial => {
                format!("{:.2}mi", ((quantity / 1000.0) * 0.621371))
            }
        }
    }

    pub fn elevation(&self, elevation: f32) -> String {
        match self.system {
            UnitSystem::Metric => {
                format!("{:.2}m", elevation)
            }
            UnitSystem::Imperial => {
                format!("{:.2}ft", elevation * 3.28084)
            }
        }
    }

    pub fn pace(&self, time: i32, distance: f32) -> String {
        match self.system {
            UnitSystem::Metric => {
                let spm = time as f32 / distance;

                format!("{} /km", self.stopwatch_time((spm * 1000.0).round() as i32))
            }
            UnitSystem::Imperial => {
                let spm = time as f32 / distance;
                format!(
                    "{} /mi",
                    self.stopwatch_time(((spm * 1000.0) / 0.621371).round() as i32)
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

    pub(crate) fn speed(&self, distance: f32, elapsed_time: i32) -> String {
        let kmph = (distance / 1000.0) / (elapsed_time as f32 / 3600.0);
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
