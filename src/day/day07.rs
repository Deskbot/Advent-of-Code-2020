use std::collections::HashMap;
use std::fs;
use substring::Substring;

pub fn day07() {
    let file = fs::read_to_string("input/day07.txt").expect("input not found");

    println!("Part 1: {}", part1(&file));
    // println!("Part 2: {}", part2(&file));
}

fn part1(input: &str) -> i32 {
    let bag_rules = input
        .lines()
        .map(|line| {
            let mut split = line.split(" contain ");
            let bag = split.next().unwrap();
            let rules = split.next().unwrap();

            return (remove_last_word(bag), parse_rules(rules));
        })
        .collect::<HashMap<&str, Vec<&str>>>();

    let mut contains_golden = HashMap::new() as HashMap<&str, bool>;

    for bag in bag_rules.keys() {
        let poop = bag_rules
            .get(bag)
            .unwrap();

        println!("{}, {:?}", bag, poop);


        let bag_deep_contains_golden = bag_rules
            .get(bag)
            .unwrap()
            .iter()
            .any(|&bag| *contains_golden.get(bag).unwrap_or(&false));

        contains_golden.insert(bag, bag_deep_contains_golden);
    }

    return contains_golden
        .values()
        .filter(|&&b| b)
        .count() as i32;
}

// fn part2(input: &str) -> i32 {

// }

fn parse_rules(rules: &str) -> Vec<&str> {
    if rules == "no other bags." {
        return Vec::new();
    }

    // remove the trailing full-stop
    let rules = rules.substring(0, rules.len() - 1);

    return rules
        .split(", ")
        .map(|rule| {
            let space = rule.find(" ").unwrap();
            let non_number_part = rule.substring(space + 1, rule.len());
            return remove_last_word(non_number_part);
        })
        .collect();
}

fn remove_last_word(string: &str) -> &str {
    let last_space = string.rfind(" ").unwrap();
    return string.substring(0, last_space);
}
