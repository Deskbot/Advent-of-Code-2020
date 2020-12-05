use std::fs;

use crate::day::day02part1;
use crate::day::day02part2;

pub fn day02() {
    let file = fs::read_to_string("input/day02.txt")
        .expect("input not found");

    let lines = file.lines();

    {
        let matching_passwords = lines.clone().filter(|line| day02part1::test_line(line));

        println!("Part 1: {}", matching_passwords.count());
    }

    {
        let matching_passwords = lines.filter(|line| day02part2::test_line(line));

        println!("Part 2: {}", matching_passwords.count());
    }
}
