use std::fs;

pub fn day11() {
    let file = fs::read_to_string("input/day11.txt").expect("input not found");

    println!("Part 1: {}", part1(&file));
    // println!("Part 2: {}", part2(&joltages));
}

fn part1(joltages: &str) -> i64 {

}
