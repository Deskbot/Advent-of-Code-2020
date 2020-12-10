use std::fs;

pub fn day10() {
    let file = fs::read_to_string("input/day10.txt").expect("input not found");

    println!("Part 1: {}", part1(&file));
    // println!("Part 2: {}", part2(&file, 552655238));
}

fn part1(input: &str) -> i32 {
    let mut chargers = input
        .lines()
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .collect::<Vec<i32>>();

    chargers.sort();

    let mut diffs_of_1 = 0;
    let mut diffs_of_3 = 0;

    for pair in chargers.windows(2) {
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
