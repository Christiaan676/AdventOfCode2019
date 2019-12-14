use std::cmp::Ordering;
use std::ops;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl ops::Sub<Coordinate> for Coordinate {
    type Output = Coordinate;

    fn sub(self, rhs: Coordinate) -> Coordinate {
        Coordinate {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Add<Coordinate> for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Coordinate) -> Coordinate {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign<Coordinate> for Coordinate {
    fn add_assign(&mut self, rhs: Coordinate) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Coordinate {
    pub fn center() -> Coordinate {
        Coordinate { x: 0, y: 0 }
    }

    pub fn new(x: i32, y: i32) -> Coordinate {
        Coordinate { x, y }
    }
}
