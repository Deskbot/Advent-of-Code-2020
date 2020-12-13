use std::collections::HashMap;
use std::fs;

pub fn day13() {
    let file = fs::read_to_string("input/day13.txt").expect("input not found");

    println!("Part 1: {}", part1(&file));
    println!("Part 2: {}", part2(&file));
}

fn part1(input: &str) -> i64 {
    let mut itr = input.lines();

    let current_time = itr.next().unwrap().parse::<i64>().unwrap();
    let buses = itr.next().unwrap()
        .split(',')
        .filter(|&bus| bus != "x")
        .map(str::parse::<i64>)
        .map(Result::unwrap);

    let bus_wait_times = buses.map(|id| {
        let last_time_arrived = -(current_time % id); // without the minus it's the additional time needed to reach current time
        let next_bus_after_current_time = last_time_arrived + id; // last_time_arrived.abs() < id

        return (id, next_bus_after_current_time)
    });

    let soonest_bus = bus_wait_times
        .min_by(|(_, wait1), (_, wait2)| wait1.cmp(wait2)) // smallest wait time
        .unwrap();

    let (bus_id, wait_time) = soonest_bus;

    return bus_id * wait_time;
}

fn part2(input: &str) -> i64 {
    let itr = input.lines();
    let mut itr = itr.skip(1); // skip current time line

    let mut buses = itr.next().unwrap()
        .split(',')
        .map(str::parse::<i64>)
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .collect::<Vec<i64>>();


    // l = list of numbers

    // start at time 0

    // go up in multiples of the first number ( the one that has to be 0 off the time i.e. a multiple of the target)

    // until we find the first 2 numbers are in order

    // the first 2 will be in order again after l[0] * l[1] steps
    // so update the value to increment by to equal this value

    // continue until we find the 1 more number in the sequence
    // etc
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "939
7,13,x,x,59,x,31,19";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 295);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 1068781);
    }

    #[test]
    fn part2_example2() {
        assert_eq!(part2("poop\n7,13"), 77);
    }

    #[test]
    fn part2_example3() {
        assert_eq!(part2("poop\n17,x,13,19"), 3417);
    }
}
