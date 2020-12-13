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

    // l = list of numbers
    let buses = itr.next().unwrap()
        .split(',')
        .map(str::parse::<i64>)
        .enumerate()
        .filter(|(_, bus_id)| bus_id.is_ok())
        .map(|(offset, bus_id)| (offset as i64, bus_id.unwrap()))
        .collect::<Vec<(i64, i64)>>();

    // start at time 0
    let mut time = 0;
    let mut inc_by = 1;

    for i in 0..buses.len()-1 {
        let (_, bus_id) = buses[i];

        inc_by *= bus_id;

        loop {
            // go up in multiples of a number
            // bus `i` will be a multiple of time
            time += inc_by;

            let (next_offset, next_bus_id) = buses[i+1];

            // until we find the next bus comes 1 minute after `i` at some time
            if (time + next_offset) % next_bus_id == 0 {
                // println!("{} {:?}", time, buses[0..i+1].iter().map(|(index,bus)| bus));
                break;
            }
        }
    };

    return time;
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

    #[test]
    fn part2_example4() {
        assert_eq!(part2("poop\n67,7,59,61"), 754018);
    }

    #[test]
    fn part2_example5() {
        assert_eq!(part2("poop\n67,x,7,59,61"), 779210);
    }

    #[test]
    fn part2_example6() {
        assert_eq!(part2("poop\n67,7,x,59,61"), 1261476);
    }

    #[test]
    fn part2_example7() {
        assert_eq!(part2("poop\n1789,37,47,1889"), 1202161486);
    }
}
