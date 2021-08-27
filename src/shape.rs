use std::convert::TryInto;
#[derive(Default, Debug, Clone, Copy)]
pub struct Position2d {
    pub x: f64,
    pub y: f64,
}

impl Position2d {
    pub fn distance(&self, other: &Position2d) -> f64 {
        f64::from((self.x - other.x).powf(2.0) + (self.y - other.y).powf(2.0)).sqrt()
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Circle {
    position: Position2d,
    radius: f64,
}

impl Circle {
    // pub fn is_inside(&self, point: &Position2d) -> bool {
    //     self.position.distance(point) <= f64::from(self.radius)
    // }
    pub fn is_overlapped(&self, other: &Circle) -> bool {
        self.position.distance(&other.position) <= f64::from(self.radius + other.radius)
    }
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        Circle {
            position: Position2d { x, y },
            radius,
        }
    }
    pub fn set_position(&mut self, x: f64, y: f64) {
        self.position = Position2d { x, y };
    }
    pub fn position(&self) -> Position2d {
        self.position
    }
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
    }
    pub fn radius(&self) -> f64 {
        self.radius
    }
    pub fn area(&self) -> f64 {
        self.radius().powf(2.0) * std::f64::consts::PI
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Rect {
    position: Position2d,
    width: f64,
    height: f64,
}

impl Rect {
    // pub fn is_inside(&self, point: &Position2d) -> bool {
    //     let (left, right, top, bottom)=(self.position.x, self.position.x+self.width as f64, self.position.y, self.position.y+self.height as f64);
    //     left<=point.x && point.x<=(right as f64) && top<=point.y && point.y<=bottom
    // }
    pub fn is_outside(&self, other: &Circle) -> bool {
        let (left, right, top, bottom): (f64, f64, f64, f64) = (
            self.position.x,
            self.position.x + self.width as f64,
            self.position.y,
            self.position.y + self.height as f64,
        );
        let center = other.position();
        match (center.x - left)
            .min(right - center.x)
            .min(center.y - top)
            .min(bottom - center.y)
            .try_into()
        {
            Ok(value) => other.radius() > value,
            Err(_) => false,
        }
    }

    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Rect {
            position: Position2d { x, y },
            width,
            height,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn position2d_distance() {
        let dot = Position2d { x: 3.0, y: 4.0 };
        let distance = dot.distance(&dot);
        assert!(
            distance.abs() < f64::EPSILON,
            "Distance between a point and itself should be 0; distance:{}",
            distance
        );
        let distance = Position2d { x: 0.0, y: 0.0 }.distance(&dot);
        assert!(
            (distance - 5.0).abs() < f64::EPSILON,
            "Distance between (3,4) and (0,0) should be 5; distance:{}",
            distance
        );
        let flipped_distance = dot.distance(&Position2d { x: 0.0, y: 0.0 });
        assert!(
            (distance - flipped_distance).abs() < f64::EPSILON,
            "Distance operation must be commutative; distance:{}",
            distance
        );
    }
    #[test]
    fn circle_overapping() {
        let origin = Circle::new(0.0, 0.0, 3.0);
        let outer = Circle::new(5.0, 5.0, 2.0);
        assert!(!origin.is_overlapped(&outer));
        let contact = Circle::new(3.0, 4.0, 2.0);
        assert!(origin.is_overlapped(&contact));
        let intersect = Circle::new(3.0, 4.0, 3.0);
        assert!(origin.is_overlapped(&intersect));
        let inner = Circle::new(0.0, 1.0, 1.0);
        assert!(origin.is_overlapped(&inner));
    }
    #[test]
    fn rect_is_outside() {
        let (x, y, w, h) = (0.0, 0.0, 640.0, 480.0);
        let board = Rect::new(x, y, w, h);
        //border tests
        let one = 1.0;
        assert!(board.is_outside(&Circle::new(x, y, one)));
        assert!(!board.is_outside(&Circle::new(x + one, y + one, one)));
        assert!(board.is_outside(&Circle::new(x, y + h, one)));
        assert!(!board.is_outside(&Circle::new(x + one, y + h - one, one)));
        assert!(board.is_outside(&Circle::new(x + w, y, one)));
        assert!(!board.is_outside(&Circle::new(x + w - one, y - one, one)));
        assert!(board.is_outside(&Circle::new(x + w, y + h, one)));
        assert!(!board.is_outside(&Circle::new(x + w - one, y + h - one, one)));
        //arbitary
        assert!(board.is_outside(&Circle::new(x + w - 32.0, y + h - 31.0, 32.0)));
    }
}
