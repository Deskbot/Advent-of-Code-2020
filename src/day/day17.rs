use std::collections::HashMap;
use std::fs;
use itertools::Itertools;

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

        for (&x, plane) in &self.space {
            for (&y, row) in plane {
                for (&z, cell) in row {
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

// impl fmt::Display for Conway3D {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

//         let mut poop = self.space.keys().collect::<Vec<&i64>>();
//         poop.sort();

//         for plane in poop {
//             self.write_plane(f, self.space.get(plane).unwrap());
//         }

//         write!(f, "({}, {})", self.x, self.y)
//     }

//     fn write_plane(&self, f: &mut fmt::Formatter<'_>, plane: &Vec<&i64>) {

//     }
// }

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
