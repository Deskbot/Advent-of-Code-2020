use std::fs;

pub fn day01() {
    let numbers = fs::read_to_string("src/input/day01.txt")
        .expect("input not found")
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let result = part1(&numbers);
    println!("Part1: {}", result);

    let result = part2(&numbers);
    println!("Part2: {}", result);
}

fn part1(numbers: &Vec<i32>) -> i32 {
    for left in numbers {
        for right in numbers {
            if left + right == 2020 {
                return left * right;
            }
        }
    }

    panic!("No pair of numbers adds up to 2020.")
}

fn part2(numbers: &Vec<i32>) -> i32 {
    for first in numbers {
        for second in numbers {
            for third in numbers {
                if first + second + third == 2020 {
                    return first * second * third;
                }
            }
        }
    }

    panic!("No trio of numbers adds up to 2020.")
}
