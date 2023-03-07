pub fn stopwatch_time(seconds: u32) -> String {
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

pub enum DistanceUnit {
    Metric,
    Imperial
}

pub fn distance(quantity: f32, unit: &DistanceUnit) -> String {
    match unit {
        DistanceUnit::Metric => {
            format!("{:.2}km", (quantity / 1000.0).round())
        },
        DistanceUnit::Imperial => {
            format!("{:.2}mi", ((quantity / 1000.0) * 0.621371))
        },
    }
}

pub fn elevation(elevation: f32, unit: &DistanceUnit) -> String {
    match unit {
        DistanceUnit::Metric => {
            format!("{:.2}m", elevation)
        }
        DistanceUnit::Imperial => {
            format!("{:.2}ft", elevation * 3.28084)
        }
    }
}

pub fn pace(elapsed_time: u32, distance: f32, unit: &DistanceUnit) -> String {
    match unit {
        DistanceUnit::Metric => {
            let spm = elapsed_time as f32 / distance;

            format!("{} /km", stopwatch_time((spm * 1000.0).round() as u32))
        }
        DistanceUnit::Imperial => {
            let spm = elapsed_time as f32 / distance;
            format!("{} /mi", stopwatch_time(((spm * 1000.0) / 0.621371).round() as u32))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::time_format::{DistanceUnit, pace};

    use super::stopwatch_time;

    #[test]
    fn test_stopwatch_time() {
        assert_eq!("00:00", stopwatch_time(0));
        assert_eq!("00:30", stopwatch_time(30));
        assert_eq!("01:00", stopwatch_time(60));
        assert_eq!("01:00:00", stopwatch_time(3600));
    }

    #[test]
    fn test_pace() {
        assert_eq!("02:00", pace(120, 1000.0, &DistanceUnit::Metric));
    }
}
