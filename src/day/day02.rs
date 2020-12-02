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

impl Range {
    pub fn contains(&self, num: i32) -> bool {
        num >= self.min && num <= self.max
    }
}

struct Rule {
    range: Range,
    letter: char,
}

impl Rule {
    pub fn test(&self, password: &str) -> bool {
        let num_matching_chars = password.chars()
            .filter(|&c| c == self.letter)
            .count();

        self.range.contains(num_matching_chars as i32)
    }
}

pub fn day02() {
    let file = fs::read_to_string("input/day02.txt")
        .expect("input not found");

    let lines = file.lines();

    let things_to_test = lines.map(test_line);

    let matching_passwords = things_to_test
        .filter(|&b| b)
        .count();

    println!("Part 1: {}", matching_passwords);
}

fn test_line(line: &str) -> bool {
    let (rule, password) = input_to_test(line);
    return rule.test(password);
}

fn input_to_test(line: &str) -> (Rule, &str) {
    let mut itr = line.split(": ");
    let rule = itr.next();
    let password = itr.next();

    let parsed_rule = rule.map(parse_rule);

    both(parsed_rule, password).unwrap()
}

fn parse_rule(rule: &str) -> Rule {
    let mut itr = rule.split(" ");

    let range = itr.next();
    let letter = option_bind(
        itr.next(),
        |letter| letter.chars().nth(0)
    );

    both(range, letter).and_then(|(range, letter)| {
        Some(Rule {
            range: parse_range(range),
            letter,
        })
    })
    .expect(format!("Couldn't parse rule: {}", rule).as_str())
}

fn parse_range(range: &str) -> Range {
    let mut itr = range.split("-");
    let min = option_bind(itr.next(), |num| num.parse::<i32>().ok());
    let max = option_bind(itr.next(), |num| num.parse::<i32>().ok());

    both(min, max)
        .and_then(|(min,max)| {
            Some(Range {
                min,
                max,
            })
        })
        .expect(format!("Couldn't parse range: {}", range).as_str())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert!(test_line("1-3 a: abcde"));
        assert!(!test_line("1-3 b: cdefg"));
        assert!(test_line("2-9 c: ccccccccc"));
    }
}
