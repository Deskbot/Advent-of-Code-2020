use regex::Regex;
use std::fs;

struct Range {
    min: i64,
    max: i64,
}

struct Rule {
    name: String,
    // luckily it looks like these ranges don't overlap
    range_1: Range,
    range_2: Range
}

impl Rule {

    pub fn parse(s: &str) -> Rule {
        let reg = Regex::new(r"([a-z ]+): ([0-9]+)-([0-9]+) or ([0-9]+)-([0-9]+)").unwrap();
        let itr = reg.captures(s).unwrap();

        let name = itr.get(2).unwrap().as_str().to_string();
        let range_1 = Range {
            min: itr.get(2).unwrap().as_str().parse::<i64>().unwrap(),
            max: itr.get(3).unwrap().as_str().parse::<i64>().unwrap(),
        };
        let range_2 = Range {
            min: itr.get(4).unwrap().as_str().parse::<i64>().unwrap(),
            max: itr.get(5).unwrap().as_str().parse::<i64>().unwrap(),
        };

        return Rule {
            name,
            range_1,
            range_2,
        }
    }
}

pub fn day16() {
    let file = fs::read_to_string("input/day16.txt").expect("input not found");

    println!("Part 1: {}", part1(&file));
    // println!("Part 2: {}", part2(&file));
}

fn part1(input: &str) -> i64 {
    let mut chunks = input.split("\n\n");
    let rules = chunks.next().unwrap();
    let my_ticket = chunks.next().unwrap();
    let nearby_tickets = chunks.next().unwrap();

    let rules = rules
        .split("\n")
        .map(Rule::parse);


    0
}
