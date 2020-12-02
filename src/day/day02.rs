use std::fs;

use crate::day::day02part1;
use crate::day::day02part2;

pub fn day02() {
    let file = fs::read_to_string("input/day02.txt")
        .expect("input not found");

    let lines = file.lines();

    {
        let things_to_test = lines.clone().map(day02part1::test_line);

        let matching_passwords = things_to_test
            .filter(|&b| b)
            .count();

        println!("Part 1: {}", matching_passwords);
    }

    {
        let things_to_test = lines.map(day02part2::test_line);
        let matching_passwords = things_to_test
            .filter(|&b| b)
            .count();

        println!("Part 2: {}", matching_passwords);
    }
}
