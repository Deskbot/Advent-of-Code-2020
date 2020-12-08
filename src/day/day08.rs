use crate::console::{
    parse_code,
    Console,
};
use std::collections::HashSet;
use std::fs;

pub fn day08() {
    let file = fs::read_to_string("input/day08.txt").expect("input not found");

    println!("Part 1: {}", part1(&file));
    // println!("Part 2: {}", part2(&file));
}

fn part1(input: &str) -> i32 {
    let instructions = parse_code(input);
    let mut machine = Console::new(instructions);

    let mut prev_addresses = HashSet::new();

    while !prev_addresses.contains(&machine.address) {
        prev_addresses.insert(machine.address);
        machine.step();
    }

    return machine.accumulator;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = fs::read_to_string("input/day08.txt").expect("input not found");
        assert_eq!(part1(&input), 1766);
    }
}
