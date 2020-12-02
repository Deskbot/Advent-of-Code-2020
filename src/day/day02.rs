use std::fs;

pub fn day02() {
    let numbers = fs::read_to_string("src/input/day02.txt")
        .expect("input not found")
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();


}
