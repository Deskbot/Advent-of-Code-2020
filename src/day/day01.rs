use std::fs;

pub fn day01() {
    let numbers = fs::read_to_string("src/input/day01_part1.txt")
        .expect("input not found")
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let result = do_maths(&numbers);

    println!("{}", result);
}

fn do_maths(numbers: &Vec<i32>) -> i32 {
    for left in numbers {
        for right in numbers {
            if left + right == 2020 {
                return left * right;
            }
        }
    }

    panic!("No pair of numbers adds up to 2020.")
}
