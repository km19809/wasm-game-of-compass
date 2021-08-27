use std::fmt;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    LightRed,
    LightGreen,
    LightBlue,
    LightYellow,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color_str = match self {
            Color::Red => "#e5614a",
            Color::Green => "#5cd074",
            Color::Blue => "#4aa3e3",
            Color::Yellow => "#f5ca1a",
            Color::LightRed => "#e69d91",
            Color::LightGreen => "#99d1a4",
            Color::LightBlue => "#91c1e3",
            Color::LightYellow => "#f5de84",
        };
        write!(f, "{}", color_str)
    }
}
impl Color {
    pub fn to_light(&self) -> Color {
        match self {
            Color::Red => Color::LightRed,
            Color::Green => Color::LightGreen,
            Color::Blue => Color::LightBlue,
            Color::Yellow => Color::LightYellow,
            _ => *self,
        }
    }
    pub fn to_dark(&self) -> Color {
        match self {
            Color::LightRed => Color::Red,
            Color::LightGreen => Color::Green,
            Color::LightBlue => Color::Blue,
            Color::LightYellow => Color::Yellow,
            _ => *self,
        }
    }
    pub fn next(&self) -> Color {
        match self {
            Color::Red => Color::Green,
            Color::Green => Color::Blue,
            Color::Blue => Color::Yellow,
            Color::Yellow => Color::Red,
            Color::LightRed => Color::LightGreen,
            Color::LightGreen => Color::LightBlue,
            Color::LightBlue => Color::LightYellow,
            Color::LightYellow => Color::LightRed,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lighten() {
        assert_eq!(Color::Red.to_light(), Color::LightRed);
        assert_eq!(Color::Green.to_light(), Color::LightGreen);
        assert_eq!(Color::Blue.to_light(), Color::LightBlue);
        assert_eq!(Color::Yellow.to_light(), Color::LightYellow);
        assert_eq!(Color::LightRed.to_light(), Color::LightRed);
        assert_eq!(Color::LightGreen.to_light(), Color::LightGreen);
        assert_eq!(Color::LightBlue.to_light(), Color::LightBlue);
        assert_eq!(Color::LightYellow.to_light(), Color::LightYellow);
    }
    #[test]
    fn darken() {
        assert_eq!(Color::Red.to_dark(), Color::Red);
        assert_eq!(Color::Green.to_dark(), Color::Green);
        assert_eq!(Color::Blue.to_dark(), Color::Blue);
        assert_eq!(Color::Yellow.to_dark(), Color::Yellow);
        assert_eq!(Color::LightRed.to_dark(), Color::Red);
        assert_eq!(Color::LightGreen.to_dark(), Color::Green);
        assert_eq!(Color::LightBlue.to_dark(), Color::Blue);
        assert_eq!(Color::LightYellow.to_dark(), Color::Yellow);
    }
    #[test]
    fn next() {
        assert_eq!(Color::Red.next(), Color::Green);
        assert_eq!(Color::Green.next(), Color::Blue);
        assert_eq!(Color::Blue.next(), Color::Yellow);
        assert_eq!(Color::Yellow.next(), Color::Red);
        assert_eq!(Color::LightRed.next(), Color::LightGreen);
        assert_eq!(Color::LightGreen.next(), Color::LightBlue);
        assert_eq!(Color::LightBlue.next(), Color::LightYellow);
        assert_eq!(Color::LightYellow.next(), Color::LightRed);
    }
}
