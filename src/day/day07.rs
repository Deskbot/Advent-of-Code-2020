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

    for &bag in bag_rules.keys() {
        let bag_deep_contains_golden = bag_rules
            .get(bag)
            .unwrap()
            .iter()
            .any(|&rule| rule == "shiny gold" || *contains_golden.get(bag).unwrap_or(&false));

        contains_golden.insert(bag, bag_deep_contains_golden);

        println!("{} {}", bag, bag_deep_contains_golden);
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


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.\n\
                                dark orange bags contain 3 bright white bags, 4 muted yellow bags.\n\
                                bright white bags contain 1 shiny gold bag.\n\
                                muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\n\
                                shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\n\
                                dark olive bags contain 3 faded blue bags, 4 dotted black bags.\n\
                                vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\n\
                                faded blue bags contain no other bags.\n\
                                dotted black bags contain no other bags.";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 357);
    }

    #[test]
    fn test() {
        assert_eq!(part1("bright white bags contain 1 shiny gold bag.\n"), 1);
    }
}
