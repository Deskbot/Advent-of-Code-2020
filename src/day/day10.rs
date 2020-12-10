use std::collections::HashMap;
use std::fs;

pub fn day10() {
    let file = fs::read_to_string("input/day10.txt").expect("input not found");

    let joltages = input_to_joltages(&file);

    println!("Part 1: {}", part1(&joltages));
    println!("Part 2: {}", part2(&joltages));
}

fn part1(joltages: &Vec<i64>) -> i64 {
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

fn part2(joltages: &Vec<i64>) -> i64 {

    let joltage_dependencies = (0..joltages.len() - 1) // don't check dependencies of my device
        .map(|index| {
            let joltage = joltages[index];
            // get a lazy list of joltages larger than the one we are looking at
            // anything after the current index will be larger
            let deps = can_connect(joltage, &joltages[(index + 1)..]);

            return (joltage, deps);
        });

    // strategy:
    // start with the larger adapters and memoise how many chains to my device can start with each adapter
    // no adapter will depend on a smaller adapter

    let mut ways_of_adding_to = HashMap::<i64, i64>::new(); // joltage to qty

    let &my_device = joltages.last().unwrap();
    ways_of_adding_to.insert(my_device, 1); // exactly 1 chain that starts with my device and ends with my device

    for (joltage, deps) in joltage_dependencies.rev() {

        let ways = deps.iter()
            .map(|dep_jolt| ways_of_adding_to.get(&dep_jolt).unwrap())
            .fold(0, |acc, sum| acc + sum);

        ways_of_adding_to.insert(joltage, ways);
    }

    return *ways_of_adding_to.get(&0).unwrap();
}

fn can_connect(joltage: i64, might_connect: &[i64]) -> Vec<i64> {
    let mut results = Vec::with_capacity(3);

    for &connector in might_connect {
        if connector - joltage <= 3 {
            results.push(connector);
        } else {
            // the connectors are in order, so no further connector will connect
            break;
        }
    }

    return results;
}

fn input_to_joltages(input: &str) -> Vec<i64> {
    let mut joltages = input
        .lines()
        .map(str::parse::<i64>)
        .map(Result::unwrap)
        .collect::<Vec<i64>>();

    // add the socket
    joltages.insert(0, 0);

    joltages.sort();

    // add my device
    let &biggest_charger = joltages.last().unwrap();
    joltages.push(biggest_charger + 3);

    return joltages;
}


#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(part1(&input_to_joltages(EXAMPLE_1)), 35);
    }

    #[test]
    fn part1_example_2() {
        assert_eq!(part1(&input_to_joltages(EXAMPLE_2)), 220);
    }

    #[test]
    fn part1_answer() {
        let file = fs::read_to_string("input/day10.txt").expect("input not found");
        let joltages = input_to_joltages(&file);
        assert_eq!(part1(&joltages), 2376);
    }

    #[test]
    fn part2_example_1() {
        assert_eq!(part2(&input_to_joltages(EXAMPLE_1)), 8);
    }

    #[test]
    fn part2_example_2() {
        assert_eq!(part2(&input_to_joltages(EXAMPLE_2)), 19208);
    }
}

