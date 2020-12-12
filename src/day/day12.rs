use std::fs;
use crate::point::Point;

#[derive(Eq, PartialEq)]
enum Action {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

impl Action {
    pub fn from_char(c: char) -> Action {
        match c {
            'N' => Action::North,
            'S' => Action::South,
            'E' => Action::East,
            'W' => Action::West,
            'L' => Action::Left,
            'R' => Action::Right,
            'F' => Action::Forward,
            _ => panic!("invalid character"),
        }
    }
}

struct Move {
    pub action: Action,
    pub magnitude: i64,
}

impl Move {
    pub fn new(action: Action, magnitude: i64) -> Move {
        Move {
            action,
            magnitude,
        }
    }
}

struct Ship {
    position: Point,
    angle: i64, // degrees
}

impl Ship {
    pub fn new(position: Point, angle: i64) -> Ship {
        Ship {
            position,
            angle,
        }
    }

    pub fn go(&mut self, m: &Move) {
        use Action::*;

        if m.action == Left {
            self.angle -= m.magnitude;
            return;
        } else if m.action == Right {
            self.angle += m.magnitude;
            return;
        }

        let displacement =
            if m.action == Forward {
                match self.angle % 360 {
                      0 => Point::new( 0,           m.magnitude),
                     90 => Point::new( m.magnitude, 0),
                    180 => Point::new( 0,          -m.magnitude),
                    270 => Point::new(-m.magnitude, 0),
                    _ => panic!("wtf"),
                }
            } else {
                match m.action {
                    North => Point::new( 0,           m.magnitude),
                    East  => Point::new( m.magnitude, 0),
                    South => Point::new( 0,          -m.magnitude),
                    West  => Point::new(-m.magnitude, 0),
                    _ => panic!("wtf"),
                }
            };

        self.position.plus(&displacement);
    }

    pub fn manhattan_distance(&self) -> i64 {
        return self.position.x + self.position.y;
    }
}

pub fn day12() {
    let file = fs::read_to_string("input/day12.txt").expect("input not found");

    println!("Part 1: {}", part1(&file));
    // println!("Part 2: {}", part2(&file));
}

fn part1(input: &str) -> i64 {

    // parse input into a list of directions
    let directions = parse_input(input);

    // start at 0,0
    let mut ship = Ship::new(Point::new(0,0), 90); // 90 i.e. 3 o'clock i.e. East

    // for each direction alter the direction of the current location
    for direction in directions {
        ship.go(&direction);
    }

    // return get manhattan distance from 0,0 to current position

    return ship.manhattan_distance();
}

fn parse_input(input: &str) -> Vec<Move> {
    input.lines()
        .map(parse_direction)
        .collect()
}

fn parse_direction(s: &str) -> Move {
    let mut iter = s.chars();
    let orientation_code = iter.next().unwrap();
    let magnitude_str = iter.collect::<String>();

    let orientation = Action::from_char(orientation_code);
    let magnitude = magnitude_str.parse::<i64>().unwrap();
    return Move::new(orientation, magnitude);
}
