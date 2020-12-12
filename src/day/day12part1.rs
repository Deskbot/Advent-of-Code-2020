use crate::point::Point;
use crate::Action;

pub struct Ship {
    pub position: Point,
    angle: i64, // degrees
}

impl Ship {
    pub fn new(position: Point, angle: i64) -> Ship {
        Ship {
            position,
            angle,
        }
    }

    pub fn go(&mut self, action: &Action) {
        use Action::*;

        if let Left(mag) = action {
            self.angle -= mag;
            return;
        } else if let Right(mag) = action {
            self.angle += mag;
            return;
        }

        let displacement =
            if let &Forward(mag) = action {
                match self.angle % 360 {
                      0        => Point::new( 0, mag),
                     90 | -270 => Point::new( mag, 0),
                    180 | -180 => Point::new( 0, -mag),
                    270 |  -90 => Point::new(-mag, 0),
                    _ => panic!("wtf {}", self.angle),
                }
            } else {
                match *action {
                    North(mag) => Point::new( 0, mag),
                    East(mag)  => Point::new( mag, 0),
                    South(mag) => Point::new( 0, -mag),
                    West(mag)  => Point::new(-mag, 0),
                    _ => panic!("wtf"),
                }
            };

        self.position = self.position.plus(&displacement);
    }

    pub fn manhattan_distance(&self) -> i64 {
        return self.position.x.abs() + self.position.y.abs();
    }
}
