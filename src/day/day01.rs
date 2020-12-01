use std::fs;

pub fn day01() {
    let numbers = fs::read_to_string("src/input/day01_part1.txt")
        .expect("input not found")
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    for num in &numbers {
        println!("{}", num);
    }

}
