use std::collections::HashMap;
use std::fs;

#[derive(Clone)]
enum State {
    Alive,
    Dead,
}

impl State {
    pub fn parse(c: &char) -> State {
        match c {
            '#' => State::Alive,
            '.' => State::Dead,
            _ => panic!("invalid char"),
        }
    }
}

struct Conway3D {
    space: HashMap::<i64, HashMap<i64, HashMap<i64, State>>>
}

impl Conway3D {
    pub fn new() -> Conway3D {
        Conway3D {
            space: HashMap::new()
        }
    }

    fn get(&mut self, x: i64, y: i64, z: i64) -> &State {
        self.space
            .entry(x).or_insert(HashMap::new())
            .entry(y).or_insert(HashMap::new())
            .entry(z).or_insert(State::Dead)
    }

    fn insert(&mut self, state: State, x: i64, y: i64, z: i64) {
        self.space
            .entry(x).or_insert(HashMap::new())
            .entry(y).or_insert(HashMap::new())
            .insert(z, state);
    }

    pub fn insert_plane(&mut self, plane: Vec<Vec<State>>) {
        for x in 0..plane.len() {
            let row = plane.get(x).unwrap();
            for y in 0..row.len() {
                let cell = row.get(y).unwrap();
                self.insert(cell.clone(), x as i64, y as i64, 0)
            }
        }
    }
}

pub fn day17() {
    let file = fs::read_to_string("input/day16.txt").expect("input not found");

    println!("Part 1: {}", part1(&file));
    // println!("Part 2: {}", part2(&file));
}

fn part1(input: &str) -> i64 {
    // parse input
    let input_plane = input.split("\n\n")
        .map(
            |line| line.chars()
                .map(|c| State::parse(&c))
                .collect()
        )
        .collect::<Vec<Vec<State>>>();

    // make conway structure
    let mut conway = Conway3D::new();

    // insert plane
    conway.insert_plane(input_plane);

    // increment 6 times

    // return count alive

    0
}
