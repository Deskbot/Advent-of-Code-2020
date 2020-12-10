use std::fs;

pub fn day10() {
    let file = fs::read_to_string("input/day10.txt").expect("input not found");

    println!("Part 1: {}", part1(&file));
    // println!("Part 2: {}", part2(&file, 552655238));
}

fn part1(input: &str) -> i32 {
    let chargers = input
        .lines()
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .collect::<Vec<i32>>()
        .sort();

    0
}
