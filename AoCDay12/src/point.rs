use std::ops;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl ops::Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Point {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.y + rhs.z,
        }
    }
}

impl ops::AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Default for Point {
    fn default() -> Self {
        Point::new(0, 0, 0)
    }
}

impl Point {
    pub fn new(x: i32, y: i32, z: i32) -> Point {
        Point { x, y, z }
    }
}
