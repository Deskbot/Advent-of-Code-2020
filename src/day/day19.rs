use std::collections::HashMap;
use std::str::Chars;
use std::fs;

pub fn day19() {
    let file = fs::read_to_string("input/day19.txt").expect("input not found");

    println!("Part 1: {}", part1(&file));
    // println!("Part 2: {}", part2(&file));
}

fn part1(input: &str) -> i64 {
    // split into rules text and message text

    // split into list of rules
    // map parse rules

    // create a hashmap of rule numbers to rules


    // parse messages

    // map by passes rule 0

    // return

    0
}

enum Step {
    SubRule(i64),
    Char(char)
}

impl Step {
    pub fn parse(s: &str) -> Step {
        // parse int
        // success: Symbol

        // fail: take 2nd char from string, return Char
    }

    pub fn pass(&self, rules: &HashMap<i64,Rule>, s: &mut &str) -> bool {
        // if I want a char, check the char
        // else call pass on the SubRule referenced
    }
}

type Sequence = Vec<Step>;

impl Sequence {
    pub fn parse(s: &str) -> Sequence {
        // split each subrule string by ' ' to get a list of step strings
        // parse string into step
    }

    pub fn pass(&self, rules: &HashMap<i64,Rule>, s: &str) -> bool {
        // return true if all steps pass
    }
}

struct Rule {
    number: i64,
    sequences: Vec<Sequence>,
}

impl Rule {
    pub fn parse(s: &str) -> Rule {
        // split by ": "
        // left is number

        // right split by | to get a list of subrules strings

        Rule {
            number,
            sequences,
        }
    }

    pub fn pass(&self, rules: &HashMap<i64,Rule>, s: &str) -> bool {
        // return true if any sub rule passes
    }

}

