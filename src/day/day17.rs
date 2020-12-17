use std::collections::HashMap;
use std::fs;

#[derive(Clone, Debug, Eq, PartialEq)]
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

#[derive(Debug)]
struct Conway3D {
    space: HashMap::<i64, HashMap<i64, HashMap<i64, State>>>,
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
    min_z: i64,
    max_z: i64,
}

impl Conway3D {
    pub fn new() -> Conway3D {
        Conway3D {
            space: HashMap::new(),
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
            min_z: 0,
            max_z: 0,
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

    fn get(&self, x: i64, y: i64, z: i64) -> &State {
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
        if x < self.min_x {
            self.min_x = x;
        } else if x > self.max_x {
            self.max_x = x;
        }

        if y < self.min_y {
            self.min_y = y;
        } else if y < self.max_y {
            self.max_y = y;
        }

        if z < self.min_z {
            self.min_z = z;
        } else if z < self.max_z {
            self.max_z = z;
        }

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

    fn neighbours(&self, x: i64, y: i64, z: i64) -> Vec<&State> {
        vec![
            self.get(x-1,y-1,z-1),
            self.get(x-1,y-1,z),
            self.get(x-1,y-1,z+1),
            self.get(x-1,y,  z-1),
            self.get(x-1,y,  z),
            self.get(x-1,y,  z+1),
            self.get(x-1,y+1,z-1),
            self.get(x-1,y+1,z),
            self.get(x-1,y+1,z+1),

            self.get(x,  y-1,z-1),
            self.get(x,  y-1,z),
            self.get(x,  y-1,z+1),
            self.get(x,  y,  z-1),
         // self.get(x,  y,  z),
            self.get(x,  y,  z+1),
            self.get(x,  y+1,z-1),
            self.get(x,  y+1,z),
            self.get(x,  y+1,z+1),

            self.get(x+1,y-1,z-1),
            self.get(x+1,y-1,z),
            self.get(x+1,y-1,z+1),
            self.get(x+1,y,  z-1),
            self.get(x+1,y,  z),
            self.get(x+1,y,  z+1),
            self.get(x+1,y+1,z-1),
            self.get(x+1,y+1,z),
            self.get(x+1,y+1,z+1),
        ]
    }

    pub fn step(&mut self) -> Conway3D {
        let mut c = Conway3D::new();

        // look at every recorded cell
        // and every cell 1 step outside the bounds of what's known
        // because those cells could change during this step
        for x in (self.min_x-1)..=(self.max_x+1) {
            for y in (self.min_y-1)..=(self.max_y+1) {
                for z in (self.min_z-1)..=(self.max_z+1) {
                    let cell = self.get(x,y,z);
                    let neighbours = self.neighbours(x,y,z);
                    let alive_neighbours = neighbours.into_iter()
                        .filter(|n| **n == State::Alive)
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
    let file = fs::read_to_string("input/day17.txt").expect("input not found");

    println!("Part 1: {}", part1(&file));
    // println!("Part 2: {}", part2(&file));
}

fn part1(input: &str) -> i64 {
    // parse input
    let input_plane = input.split("\n")
        .map(
            |line| line.chars()
                .map(|c| State::parse(&c))
                .collect()
        )
        .collect::<Vec<Vec<State>>>();

    // make conway structure
    let mut conway = Conway3D::new();

    println!("{:?}", conway);

    // insert plane
    conway.insert_plane(input_plane);

    println!("{:?}", conway);

    // increment 6 times
    for _ in 1..=6 {
        conway = conway.step();
        println!("{:?}", conway);
    }

    // return count alive
    return conway.alive_count();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(".#.
..#
###"),
            112
        );
    }
}
