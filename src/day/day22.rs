use std::fs;

pub fn day22() {
    let file = fs::read_to_string("input/day22.txt").expect("input not found");

    println!("Part 1: {}", part1(&file));
    // println!("Part 2: {}", part2(&file));
}

fn part1(input: &str) -> i64 {
    0
}
