use crate::point::Point;
use crate::Action;

pub struct Ship {
    pub position: Point,
    pub waypoint: Point,
}

impl Ship {
    pub fn new(position: Point, waypoint: Point) -> Ship {
        Ship {
            position,
            waypoint,
        }
    }

    pub fn go(&mut self, action: &Action) {
        use Action::*;

        if let &Left(mag) = action {
            self.waypoint = self.waypoint.rotate(-mag);
            return;
        } else if let &Right(mag) = action {
            self.waypoint = self.waypoint.rotate(mag);
            return;
        }

        if let &Forward(mag) = action {
            self.position = self.waypoint.multiply(mag).plus(&self.position);

        } else {
            let displacement = match *action {
                North(mag) => Point::new( 0, mag),
                East(mag)  => Point::new( mag, 0),
                South(mag) => Point::new( 0, -mag),
                West(mag)  => Point::new(-mag, 0),
                _ => panic!("wtf"),
            };

            self.waypoint = self.waypoint.plus(&displacement);
        }
    }

    pub fn manhattan_distance(&self) -> i64 {
        return self.position.x.abs() + self.position.y.abs();
    }
}
