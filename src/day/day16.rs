use regex::Regex;
use std::{collections::HashMap, fs};

#[derive(Clone,Debug)]
struct Range {
    min: i64,
    max: i64,
}

impl Range {
    pub fn within(&self, n: i64) -> bool {
        self.min <= n && n <= self.max
    }
}

#[derive(Clone,Debug)]
struct Rule {
    pub name: String,
    // luckily it looks like these ranges don't overlap
    range_1: Range,
    range_2: Range
}

impl Rule {
    pub fn parse(s: &str) -> Rule {
        let reg = Regex::new(r"([a-z ]+): ([0-9]+)-([0-9]+) or ([0-9]+)-([0-9]+)").unwrap();
        let itr = reg.captures(s).unwrap();

        let name = itr.get(1).unwrap().as_str().to_string();
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
#[derive(Clone, Debug)]
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

    pub fn len(&self) -> usize {
        self.numbers.len()
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

    // println!("{:?}", rules);

    // filter invalid tickets
    let nearby_tickets = nearby_tickets.into_iter()
        .filter(|ticket| ticket.is_valid(&rules));

    let mut field_name_index = HashMap::<String,usize>::new();

    // println!("{:?}", rules.iter().map(|rule| &rule.name).collect::<Vec<&String>>());

    let field_count = my_ticket.len();
    for index in 0..field_count {
        println!("\n\n{}", index);
        // look at all of a single field across all tickets

        // println!("{:?}", rules.iter().map(|rule| &rule.name).collect::<Vec<&String>>());

        let mut fields = nearby_tickets.clone().map(|ticket| ticket.get(index));

        // if index == 13 {

        //     println!("Fields: {:?}", fields.clone().collect::<Vec<i64>>());

        //     println!("{:?}", rules.iter().map(|rule| {
        //         let poop = fields.clone().map(|field| (field, rule.within(field)))
        //             .collect::<Vec<(i64, bool)>>();

        //         (&rule.name, poop)
        //     }).collect::<Vec<(&String, Vec<(i64,bool)>)>>());
        // }

        // get all the rules that this field is within on all tickets
        let matching_rules = rules.iter()
            .filter(|rule| {
                let poop = fields.clone().all(|field| rule.within(field));
                println!("{:?} {}", rule, poop);

                // let mut poop2 = true;
                // for field in fields {
                //     if !rule.within(field) {
                //         poop2 = false;
                //         break;
                //     }
                // }

                poop
                // poop2
            })
            .collect::<Vec<&Rule>>();

        // println!("{:?}", matching_rules.iter().map(|rule|&rule.name).collect::<Vec<&String>>());
        // assert_eq!(matching_rules.len(), 1);

        // println!("{:?}", rules.iter().map(|rule| &rule.name).collect::<Vec<&String>>());

        let &matching_rule = matching_rules.first().unwrap();

        // println!("{:?}", rules.iter().map(|rule| &rule.name).collect::<Vec<&String>>());

        // assert!(field_name_index.get(&matching_rule.name).is_none());

        field_name_index.insert(matching_rule.name.clone(), index);

        // println!("{:?}\n\n\n\n\n\n\n", rules.iter().map(|rule| &rule.name).collect::<Vec<&String>>());println!("{:?}", rules.len());
    }

    // hope that they all only match 1.

    // otherwise write more code

    // multiply the departure fields

    return
        field_name_index.keys()
            .filter(|name| name.starts_with("departure"))
            .map(|name| field_name_index.get(name).unwrap())
            .map(|&index| my_ticket.get(index))
            .fold(1, |acc, next| acc * next);
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
        .filter(|line| !line.is_empty()) // this is needed for some reason
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

    #[test]
    fn rule_1() {
        let result = Rule::parse("departure date: 38-55 or 74-952").within(56);
        assert!(!result);
    }
}
