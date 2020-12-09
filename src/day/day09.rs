use std::collections::VecDeque;
use std::fs;

pub fn day09() {
    let file = fs::read_to_string("input/day09.txt").expect("input not found");

    println!("Part 1: {}", part1(&file, 25));
    // println!("Part 2: {}", part2(&file));
}

fn part1(input: &str, preamble_size: usize) -> i32 {
    let data_stream = input.lines()
        .map(str::parse::<i32>)
        .map(Result::unwrap);

    let preamble = data_stream.clone().take(preamble_size);
    let rest = data_stream.skip(preamble_size);

    let mut pool = preamble.collect::<VecDeque<i32>>();

    for datum in rest {
        if !contains_sum_to(pool.clone().make_contiguous(), datum) {
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


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "35\n\
                           20\n\
                           15\n\
                           25\n\
                           47\n\
                           40\n\
                           62\n\
                           55\n\
                           65\n\
                           95\n\
                          102\n\
                          117\n\
                          150\n\
                          182\n\
                          127\n\
                          219\n\
                          299\n\
                          277\n\
                          309\n\
                          576\n";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE, 5), 127);
    }

    #[test]
    fn contains_sum_to_1() {
        assert!(contains_sum_to(&mut [20, 15, 25, 47, 40], 62));
    }
}
