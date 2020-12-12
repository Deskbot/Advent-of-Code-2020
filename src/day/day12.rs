use std::fs;
use crate::point::Point;
use crate::day::day12part1;
use crate::day::day12part2;

#[derive(Eq, PartialEq)]
pub enum Action {
    North(i64),
    South(i64),
    East(i64),
    West(i64),
    Left(i64),
    Right(i64),
    Forward(i64),
}

impl Action {
    pub fn parse(s: &str) -> Action {
        let mut iter = s.chars();
        let orientation_code = iter.next().unwrap();
        let magnitude_str = iter.collect::<String>();

        let magnitude = magnitude_str.parse::<i64>().unwrap();
        return Action::new(orientation_code, magnitude);
    }

    fn new(c: char, mag: i64) -> Action {
        match c {
            'N' => Action::North(mag),
            'S' => Action::South(mag),
            'E' => Action::East(mag),
            'W' => Action::West(mag),
            'L' => Action::Left(mag),
            'R' => Action::Right(mag),
            'F' => Action::Forward(mag),
            _ => panic!("invalid character"),
        }
    }
}

pub fn day12() {
    let file = fs::read_to_string("input/day12.txt").expect("input not found");

    println!("Part 1: {}", part1(&file));
    println!("Part 2: {}", part2(&file));
}

fn part1(input: &str) -> i64 {

    // parse input into a list of directions
    let directions = parse_input(input);

    // start at 0,0
    let mut ship = day12part1::Ship::new(Point::new(0,0), 90); // 90 i.e. 3 o'clock i.e. East

    // for each direction alter the direction of the current location
    for direction in directions {
        ship.go(&direction);
    }

    // return get manhattan distance from 0,0 to current position

    return ship.manhattan_distance();
}

fn part2(input: &str) -> i64 {
    let directions = parse_input(input);

    let mut ship = day12part2::Ship::new(
        Point::new(0,0),
        Point::new(10,1),
    );

    for direction in directions {
        ship.go(&direction);
    }

    return ship.manhattan_distance();
}

fn parse_input(input: &str) -> Vec<Action> {
    input.lines()
        .map(Action::parse)
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "F10
N3
F7
R90
F11";

    #[test]
    fn part1_answer() {
        let file = fs::read_to_string("input/day12.txt").expect("input not found");
        assert_eq!(part1(&file), 1424);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 286);
    }
}
