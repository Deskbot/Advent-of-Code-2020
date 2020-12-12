use std::fmt::{self, Display};
use std::fmt::Formatter;

#[derive(Clone)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Point {
        Point {x,y}
    }

    pub fn multiply(&self, n: i64) -> Point {
        Point {
            x: self.x * n,
            y: self.y * n,
        }
    }

    pub fn plus(&self, p: &Point) -> Point {
        Point {
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }

    pub fn rotate(&self, degrees: i64) -> Point {
        match degrees % 360 {
              0        => self.clone(),
             90 | -270 => Point::new(self.y, self.x),
            180 | -180 => Point::new(self.x, -self.y),
            270 |  -90 => Point::new(-self.y, self.x),
            _ => panic!("wtf {}", degrees),
        }
    }

}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.x, self.y)?;
        Result::Ok(())
    }
}