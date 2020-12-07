use std::collections::HashMap;
use std::fs;
use substring::Substring;

struct Rule<'a> (i32, &'a str);

pub fn day07() {
    let file = fs::read_to_string("input/day07.txt").expect("input not found");

    println!("Part 1: {}", part1(&file));
    println!("Part 2: {}", part2(&file));
}

fn part1(input: &str) -> i32 {
    let bag_to_rules = parse_input(input);

    let mut contains_golden = HashMap::new() as HashMap<&str, bool>;

    for &bag in bag_to_rules.keys() {
        let bag_deep_contains_golden = must_contain(bag, &bag_to_rules, &contains_golden);
        contains_golden.insert(bag, bag_deep_contains_golden);
    }

    return contains_golden
        .values()
        .filter(|&&b| b)
        .count() as i32;
}

fn must_contain(bag: &str, bag_to_rules: &HashMap<&str, Vec<Rule>>, memo: &HashMap<&str, bool>) -> bool {
    return bag_to_rules
        .get(bag)
        .unwrap()
        .iter()
        .any(|&Rule(_, rule)| {
            if rule == "shiny gold" {
                return true;
            }

            if let Some(&this_was_less_annoying_than_unwrap_or_else) = memo.get(bag) {
                return this_was_less_annoying_than_unwrap_or_else;
            }

            return must_contain(rule, bag_to_rules, memo); // this ain't memoised :((((((((((
            // need to learn lifetime parameters
        });
}

fn part2(input: &str) -> i32 {
    let bag_to_rules = parse_input(input);

    // let mut memo = HashMap::new() as HashMap<&str, i32>;

    return contains("shiny gold", &bag_to_rules, /*&mut memo*/);
}

fn contains(colour: &str, bag_to_rules: &HashMap<&str, Vec<Rule>>, /*memo: &mut HashMap<&str, i32>*/) -> i32 {
    // if let Some(&result) = memo.get(bag) {
    //     return result;
    // }

    let rules = bag_to_rules
        .get(colour)
        .unwrap()
        .into_iter();

    let inner_bags = rules.clone().count() as i32;
    let deeper_bags = rules.map(|Rule(count, colour)| count * contains(colour, bag_to_rules, /*memo*/));

    return inner_bags + deeper_bags.fold(0, |sum, next| sum + next);
}

fn parse_input(input: &str) -> HashMap<&str, Vec<Rule>>{
    input
        .lines()
        .map(|line| {
            let mut split = line.split(" contain ");
            let bag = split.next().unwrap();
            let rules = split.next().unwrap();

            return (remove_last_word(bag), parse_rules(rules));
        })
        .collect()
}

fn parse_rules(rules: &str) -> Vec<Rule> {
    if rules == "no other bags." {
        return Vec::new();
    }

    // remove the trailing full-stop
    let rules = rules.substring(0, rules.len() - 1);

    return rules
        .split(", ")
        .map(|rule| {
            let space = rule.find(" ").unwrap();
            let number = rules.substring(0, space).parse::<i32>().unwrap();
            let non_number_part = rule.substring(space + 1, rule.len());
            return Rule(number, remove_last_word(non_number_part));
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

    const EXAMPLE_PART2_INPUT: &str = "shiny gold bags contain 2 dark red bags.\n\
                                       dark red bags contain 2 dark orange bags.\n\
                                       dark orange bags contain 2 dark yellow bags.\n\
                                       dark yellow bags contain 2 dark green bags.\n\
                                       dark green bags contain 2 dark blue bags.\n\
                                       dark blue bags contain 2 dark violet bags.\n\
                                       dark violet bags contain no other bags.";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 4);
    }

     #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE_PART2_INPUT), 126);
    }

    #[test]
    fn test() {
        assert_eq!(part1("bright white bags contain 1 shiny gold bag.\n"), 1);
    }
}
