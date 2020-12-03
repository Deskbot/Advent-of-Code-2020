use std::fmt;
use std::fs;

struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn translate(&mut self, by: &Point) {
        self.x += by.x;
        self.y += by.y;
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn day03() {
    let file = fs::read_to_string("input/day03.txt")
        .expect("input not found");

    let trees = trees_hit(&file, &Point::new(3,1));
    println!("Part 1: {}", trees);

    let trees = trees_hit(&file, &Point::new(1,1))
        * trees_hit(&file, &Point::new(3,1))
        * trees_hit(&file, &Point::new(5,1))
        * trees_hit(&file, &Point::new(7,1))
        * trees_hit(&file, &Point::new(1,2));

    println!("Part 2: {}", trees);
}

fn trees_hit(input: &str, movement: &Point) -> i64 {
    let grid = input.lines().collect::<Vec<&str>>();
    let height = grid.len() as i32;
    let width = grid.get(0).unwrap().len() as i32;

    let start = Point::new(0, 0);

    let mut pos = start;
    let mut trees = 0;

    while pos.y < height {
        let x = (pos.x % width) as usize;
        let y = pos.y as usize;

        let cell = grid.get(y).unwrap().chars().nth(x).unwrap();

        if is_tree(cell) {
            trees += 1;
        }

        pos.translate(&movement);
    }

    return trees;
}

fn is_tree(c: char) -> bool {
    return c == '#';
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
        ..##.......\n\
        #...#...#..\n\
        .#....#..#.\n\
        ..#.#...#.#\n\
        .#...##..#.\n\
        ..#.##.....\n\
        .#.#.#....#\n\
        .#........#\n\
        #.##...#...\n\
        #...##....#\n\
        .#..#...#.#\n";

    #[test]
    fn part1_example() {
        assert_eq!(trees_hit(EXAMPLE_INPUT, &Point::new(3,1)), 7);
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            trees_hit(EXAMPLE_INPUT, &Point::new(1,1))
            * trees_hit(EXAMPLE_INPUT, &Point::new(3,1))
            * trees_hit(EXAMPLE_INPUT, &Point::new(5,1))
            * trees_hit(EXAMPLE_INPUT, &Point::new(7,1))
            * trees_hit(EXAMPLE_INPUT, &Point::new(1,2)),
            336
        );
    }
}
