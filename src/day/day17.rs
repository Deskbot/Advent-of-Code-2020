use std::collections::HashMap;
use std::fs;
use itertools::Itertools;

#[derive(Clone, Eq, PartialEq)]
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

    pub fn next(&self, neighbours: i64) -> State {
        let is_alive = match self {
            State::Alive => {
                neighbours == 2 || neighbours == 3
            }
            State::Dead => {
                neighbours == 3
            }
        };

        if is_alive {
            State::Alive
        } else {
            State::Dead
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

    pub fn alive_count(&self) -> i64 {
        let mut alive_count = 0;

        for plane in self.space.values() {
            for row in plane.values() {
                for cell in row.values() {
                    if *cell == State::Alive {
                        alive_count += 1;
                    }
                }
            }
        }

        return alive_count;
    }

    fn get (&self, x: i64, y: i64, z: i64) -> &State {
        let result = self.space
            .get(&x)
            .and_then(|plane| plane.get(&y))
            .and_then(|row| row.get(&z));

        if let Some(state) = result {
            return state;
        }

        return &State::Dead;
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

    fn neighbours(&self, x: i64, y: i64, z: i64) -> Vec<State> {
        // I thought writing all the combinations would make it too easy to miss one.

        let numbers = vec![-1,0,1];

        // This is a nightmare. I don't want to have to collect here.
        let poop = numbers.iter().combinations(3).collect::<Vec<Vec<&i64>>>();
        let offsets = poop
            .iter()
            .map(|list| (*list.get(0).unwrap(), *list.get(1).unwrap(), *list.get(2).unwrap()))
            .filter(|(&a,&b,&c)| !(a==0 && b==0 && c==0))
            .map(|(a,b,c)| (a.clone(),b.clone(),c.clone())) // the pain
            .collect::<Vec<(i64,i64,i64)>>();

        return offsets.into_iter()
            .map(|(x_off,y_off,z_off)| self.get(x + x_off, y + y_off, z + z_off).clone())
            .collect();
    }

    pub fn step(&mut self) -> Conway3D {
        let mut c = Conway3D::new();

        for (&x, plane) in &self.space {
            for (&y, row) in plane {
                for (&z, cell) in row {
                    let neighbours = self.neighbours(x,y,z);
                    let alive_neighbours = neighbours.into_iter()
                        .filter(|n| *n == State::Alive)
                        .count() as i64;

                    let next_state = cell.next(alive_neighbours);
                    c.insert(next_state, x, y, z);
                }
            }
        }

        return c;
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
    for _ in 1..=6 {
        conway.step();
    }

    // return count alive
    return conway.alive_count();
}
