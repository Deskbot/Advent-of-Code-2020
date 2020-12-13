use std::fs;

pub fn day13() {
    let file = fs::read_to_string("input/day13.txt").expect("input not found");

    println!("Part 1: {}", part1(&file));
    // println!("Part 2: {}", part2(&file));
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

    println!("{:?}", soonest_bus);

    return bus_id * wait_time;
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
}
