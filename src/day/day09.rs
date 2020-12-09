use std::collections::VecDeque;
use std::fs;

pub fn day09() {
    let file = fs::read_to_string("input/day09.txt").expect("input not found");

    println!("Part 1: {}", part1(&file));
    // println!("Part 2: {}", part2(&file));
}

fn part1(input: &str) -> i32 {
    let data_stream = input.lines()
        .map(str::parse::<i32>)
        .map(Result::unwrap);

    let preamble = data_stream.clone().take(25);
    let rest = data_stream.skip(25);

    let mut pool = preamble.collect::<VecDeque<i32>>();

    for datum in rest {
        if !contains_sum_to(pool.make_contiguous(), datum) {
            return datum;
        }

        pool.push_back(datum);
        pool.pop_front();
    }

    panic!("all numbers are valid");
}

fn contains_sum_to(list: &mut [i32], target_sum: i32) -> bool {
    list.sort();

    let mut lower = 0;
    let mut upper = list.len() - 1;

    // try summing 2 numbers to see if it equals the target
    // if the sum of 2 numbers is too big, we can get closer by decreasing the larger number
    // if the sum of 2 numbers is too small, we can get closer by increasing the smaller number
    // because the list is sorted, we can get the next biggest/smallest number by adjusting the index of the number by 1

    while upper > lower {
        let sum = list[lower] + list[upper];

        if sum == target_sum {
            return true;
        }

        if sum < target_sum {
            lower += 1;
        } else {
            upper -= 1;
        }
    }

    return false;
}
