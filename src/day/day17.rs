use std::fs;

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

pub fn day17() {
    let file = fs::read_to_string("input/day16.txt").expect("input not found");

    println!("Part 1: {}", part1(&file));
    // println!("Part 2: {}", part2(&file));
}

fn part1(input: &str) -> i64 {
    // make conway structure

    // parse input
    let input_plane = input.split("\n\n")
        .map(
            |line| line.chars()
                .map(|c| State::parse(&c))
                .collect()
        )
        .collect::<Vec<Vec<State>>>();

    // insert plane

    // increment 6 times

    // return count alive

    0
}
