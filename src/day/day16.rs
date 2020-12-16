use regex::Regex;
use std::{collections::HashMap, fs};

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
        self.range_1.within(n) || self.range_2.within(n)
    }
}
#[derive(Clone)]
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

    pub fn is_valid(&self, rules: &Vec<Rule>) -> bool {
        // logic copied from part1
        for &number in &self.numbers {
            let valid = rules.iter().any(|rule| rule.within(number));
            if !valid {
                return false;
            }
        }

        return true;
    }

    pub fn get(&self, index: usize) -> i64 {
        *self.numbers.get(index).unwrap()
    }
}

pub fn day16() {
    let file = fs::read_to_string("input/day16.txt").expect("input not found");

    println!("Part 1: {}", part1(&file));
    println!("Part 2: {}", part2(&file));
}

fn part1(input: &str) -> i64 {
    let (rules, _, nearby_tickets) = parse_input(input);

    let mut error_rate = 0;

    for ticket in nearby_tickets {
        for number in ticket.numbers {
            let valid = rules.iter().any(|rule| rule.within(number));
            if !valid {
                error_rate += number;
                break;
            }
        }
    }

    return error_rate;
}

fn part2(input: &str) -> i64 {
    let (rules, my_ticket, nearby_tickets) = parse_input(input);

    // filter invalid tickets
    let nearby_tickets = nearby_tickets.into_iter()
        .filter(|ticket| ticket.is_valid(&rules));


    let mut field_name_index = HashMap::<String,usize>::new();

    let field_count = 20;
    for index in 0..field_count {
        // look at all of a single field across all tickets

        let mut fields = nearby_tickets.clone().map(|ticket| ticket.get(index));

        // get all the rules that all fields of all tickets are within
        let matching_rules = rules.iter()
            .filter(|rule| fields.all(|field| rule.within(field)))
            .collect::<Vec<&Rule>>();

        assert_eq!(matching_rules.len(), 1);

        let &matching_rule = matching_rules.first().unwrap();

        assert!(field_name_index.get(&matching_rule.name).is_none());

        field_name_index.insert(matching_rule.name.clone(), index);
    }


    // hope that they all only match 1.

    // otherwise write more code

    // sum the departure fields

    return
        field_name_index.keys()
            .filter(|name| name.starts_with("departure"))
            .map(|name| field_name_index.get(name).unwrap())
            .map(|&index| my_ticket.get(index))
            .fold(0, |acc, next| acc + next);
}

fn parse_input(input: &str) -> (Vec<Rule>, Ticket, Vec<Ticket>) {
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
        .map(Ticket::parse)
        .collect();

    return (rules, my_ticket, nearby_tickets);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1("class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"),
            71
        );
    }
}
