use std::fs;

enum Seat {
    Empty,
    Floor,
    Occupied,
}

pub fn day11() {
    let file = fs::read_to_string("input/day11.txt").expect("input not found");

    println!("Part 1: {}", part1(&file));
    // println!("Part 2: {}", part2(&joltages));
}

fn part1(input: &str) -> i64 {
    let mut char_2d_vec = input.lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();


    let seat_2d_vec =
        char_2d_vec.into_iter()
            .map(|row| row.into_iter()
                .map(|ch| match ch {
                    'L' => Seat::Empty,
                    '#' => Seat::Occupied,
                    '.' => Seat::Floor,
                    _ => panic!("invalid input: char doesn't represent a seat"),
                })
                .collect::<Vec<Seat>>()
            )
            .collect::<Vec<Vec<Seat>>>();

    0
}
