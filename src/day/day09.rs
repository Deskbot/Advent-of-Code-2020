use std::collections::VecDeque;
use std::fs;

pub fn day09() {
    let file = fs::read_to_string("input/day09.txt").expect("input not found");

    println!("Part 1: {}", part1(&file, 25));
    println!("Part 2: {}", part2(&file, 552655238));
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

fn part2(input: &str, target: i64) -> i64 {
    let data_stream = input.lines()
        .map(str::parse::<i64>)
        .map(Result::unwrap)
        .collect::<Vec<i64>>();

    let mut start = 0;
    let mut end = 1;

    loop {
        let range = data_stream
            .iter()
            .skip(start)
            .take(end - start);
        let sum_of_range = range.clone().fold(0, |acc, i| acc + i);

        if sum_of_range == target {
            // sum the smallest and largest
            let mut sorted_range = range.map(|&i| i).collect::<Vec<i64>>();
            sorted_range.sort();

            return sorted_range.first().unwrap() + sorted_range.last().unwrap();
        }

        if sum_of_range < target {
            // adding more numbers will give a bigger sum
            end += 1;
        } else {
            // adding less numbers will give a smaller sum
            start += 1;
        }
    }
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
        assert_eq!(super::part1(EXAMPLE, 5), 127);
    }

    #[test]
    fn part1() {
        let file = fs::read_to_string("input/day09.txt").expect("input not found");
        assert_eq!(super::part1(&file, 25), 552655238);
    }

    #[test]
    fn part2_example() {
        assert_eq!(super::part2(EXAMPLE, 127), 62);
    }

    #[test]
    fn contains_sum_to_1() {
        assert!(contains_sum_to(&mut [20, 15, 25, 47, 40], 62));
    }
}
