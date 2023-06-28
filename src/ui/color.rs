use tui::style::Color;

pub enum ColorTheme {
    Orange,
}

impl ColorTheme {
    pub fn to_color(&self) -> Color {
        match *self {
            ColorTheme::Orange => Color::Rgb(252, 76, 2),
        }
    }
}

#[derive(Debug)]
pub struct Rgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Rgb {
    pub fn to_color(&self) -> Color {
        Color::Rgb(self.red, self.green, self.blue)
    }
}

pub fn gradiant(start: Rgb, end: Rgb, offset: f64, size: f64) -> Rgb {
    let rdiff = (end.red as f64 - start.red as f64) / size;
    let gdiff = (end.green as f64 - start.green as f64) / size;
    let bdiff = (end.blue as f64 - start.blue as f64) / size;

    Rgb {
        red: (start.red as f64 + (rdiff * offset)) as u8,
        green: (start.green as f64 + (gdiff * offset)) as u8,
        blue: (start.blue as f64 + (bdiff * offset)) as u8,
    }
}

#[cfg(test)]
mod tests {
    use super::{gradiant, Rgb};

    #[test]
    fn test_gradiant() {
        let rgb = gradiant(
            Rgb {
                red: 0,
                green: 255,
                blue: 0,
            },
            Rgb {
                red: 0,
                green: 0,
                blue: 255,
            },
            2.0,
            10.0,
        );
        panic!("{:?}", rgb);
    }
}
