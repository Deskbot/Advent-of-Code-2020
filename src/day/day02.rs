use std::fs;

use crate::util::{
    both,
    compose,
    option_bind,
};

struct Range {
    min: i32,
    max: i32
}

struct Rule {
    range: Range,
    letter: char,
}

pub fn day02() {
    let file = fs::read_to_string("input/day02.txt")
        .expect("input not found");

    let mut lines = file.lines();

    let things_to_test = lines.map(|line| {
        let mut poop = line.split(": ");
        let rule = poop.nth(0);
        let password = poop.nth(1);

        let parsed_rule = option_bind(rule, |rule| parse_rule(rule));

        both(parsed_rule, password)
            .and_then(|(parsed_rule, password)| Some((parsed_rule, password)))
    });
}

fn parse_rule(rule: &str) -> Rule {
    let mut itr = rule.split(" ");

    let range = itr.nth(0);
    let letter = itr.nth(1)
        .and_then(|letter| letter.chars().nth(0));

    both(range, letter).and_then(|(range, letter)| {
        Some(Rule {
            range: parse_range(range),
            letter,
        })
    })
    .expect("Couldn't parse rule.")
}

// option_bind(range, parse_range)

fn parse_range(range: &str) -> Range {
    let mut itr = range.split("-");
    let min = itr.nth(1).and_then(compose(str::parse::<i32>, Result::ok));
    let max = itr.nth(2).and_then(compose(str::parse::<i32>, Result::ok));

    both(min, max)
        .and_then(|(min,max)| {
            Some(Range {
                min,
                max,
            })
        })
        .expect("Couldn't parse range.")
}
