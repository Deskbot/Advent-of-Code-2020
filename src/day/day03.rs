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

    let trees = part1(&file);
    println!("Part 1: {}", trees);
}

fn part1(input: &str) -> i32 {
    let grid = input.lines().collect::<Vec<&str>>();
    let height = grid.len() as i32;
    let width = grid.get(0).unwrap().len() as i32;

    let start = Point::new(0, 0);
    let movement = Point::new(3,1);

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
    #[test]
    fn part1_example() {
       let example_input = "\
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

        assert_eq!(part1(example_input), 7);
    }
}
