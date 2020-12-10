use std::fs;

pub fn day10() {
    let file = fs::read_to_string("input/day10.txt").expect("input not found");

    println!("Part 1: {}", part1(&file));
    // println!("Part 2: {}", part2(&file, 552655238));
}

fn part1(input: &str) -> i32 {
    let mut joltages = input
        .lines()
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .collect::<Vec<i32>>();

    // add the socket
    joltages.insert(0, 0);

    joltages.sort();

    // add my device
    let &biggest_charger = joltages.last().unwrap();
    joltages.push(biggest_charger + 3);

    let mut diffs_of_1 = 0;
    let mut diffs_of_3 = 0;

    for pair in joltages.windows(2) {
        let diff = pair[1] - pair[0];

        if diff == 1 {
            diffs_of_1 += 1;
        } else if diff == 3 {
            diffs_of_3 += 1;
        } else if diff > 3 {
            panic!("You misunderstood the question.")
        }

        // else ignore and continue;
    }

    return diffs_of_1 * diffs_of_3;
}


#[cfg(test)]
mod tests {
    // use super::*;

    const EXAMPLE_1: &str = "16\n\
                             10\n\
                             15\n\
                              5\n\
                              1\n\
                             11\n\
                              7\n\
                             19\n\
                              6\n\
                             12\n\
                              4\n";

    const EXAMPLE_2: &str = "28\n\
                             33\n\
                             18\n\
                             42\n\
                             31\n\
                             14\n\
                             46\n\
                             20\n\
                             48\n\
                             47\n\
                             24\n\
                             23\n\
                             49\n\
                             45\n\
                             19\n\
                             38\n\
                             39\n\
                             11\n\
                              1\n\
                             32\n\
                             25\n\
                             35\n\
                              8\n\
                             17\n\
                              7\n\
                              9\n\
                              4\n\
                              2\n\
                             34\n\
                             10\n\
                              3\n";

    #[test]
    fn part1_example_1() {
        assert_eq!(super::part1(EXAMPLE_1), 35);
    }

    #[test]
    fn part1_example_2() {
        assert_eq!(super::part1(EXAMPLE_2), 220);
    }
}

