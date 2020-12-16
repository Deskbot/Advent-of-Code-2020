use regex::Regex;
use std::fs;

#[derive(Clone)]
struct Range {
    min: i64,
    max: i64,
}

impl Range {
    pub fn within(&self, n: i64) -> bool {
        self.min <= n && n <= self.max
    }
}

#[derive(Clone)]
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

    pub fn within(&self, n: i64) -> bool {
        self.range_1.within(n) && self.range_2.within(n)
    }
}

struct Ticket {
    numbers: Vec<i64>
}

impl Ticket {
    pub fn parse(s: &str) -> Ticket {
        let numbers = s.split(',')
            .filter(|&chunk| chunk != "") // this is needed for some reason
            .map(str::parse::<i64>)
            .map(Result::unwrap)
            .collect::<Vec<i64>>();

        return Ticket {
            numbers
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

    // parse stuff

    let rules = rules
        .split("\n")
        .map(Rule::parse)
        .collect::<Vec<Rule>>();

    let my_ticket = my_ticket.split("\n").last().unwrap();
    let my_ticket = Ticket::parse(my_ticket);

    let nearby_tickets = nearby_tickets
        .split("\n").skip(1)
        .map(Ticket::parse);

    let mut error_rate = 0;

    for ticket in nearby_tickets {
        for number in ticket.numbers {
            let invalid = rules.iter().any(|rule| !rule.within(number));
            if invalid {
                error_rate += number;
                break;
            }
        }
    }

    return error_rate;
}
