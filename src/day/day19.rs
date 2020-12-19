use std::str::Chars;
use std::collections::HashMap;
use std::fs;

pub fn day19() {
    let file = fs::read_to_string("input/day19.txt").expect("input not found");

    println!("Part 1: {}", part1(&file));
    println!("Part 2: {}", part2(&file));
}

fn part1(input: &str) -> i64 {
    // split into rules text and message text
    let mut itr = input.split("\n\n");
    let rules_str = itr.next().unwrap();
    let messages_str = itr.next().unwrap();

    // split into list of rules
    // map parse rules
    let rules = rules_str.lines().map(Rule::parse);

    // create a hashmap of rule numbers to rules
    let mut rules_map = HashMap::<i64,Rule>::new();
    for rule in rules {
        rules_map.insert(rule.number, rule);
    }

    // parse messages
    // map by passes rule 0
    // return

    return messages_str
        .lines()
        .filter(|message| {
            let mut s = message.chars();

            let rule_passes = rules_map.get(&0).unwrap()
                .pass(&mut s, &rules_map);

            // rule passes and there's no more characters that need checking
            return rule_passes && s.count() == 0;
        })
        .count() as i64;
}


fn part2(input: &str) -> i64 {
    let mut itr = input.split("\n\n");
    let rules_str = itr.next().unwrap();
    let messages_str = itr.next().unwrap();

    let rules = rules_str.lines().map(Rule::parse);

    // create a hashmap of rule numbers to rules
    let mut rules_map = HashMap::<i64,Rule>::new();
    for rule in rules {
        rules_map.insert(rule.number, rule);
    }

    // now modify the incorrect rules and put them in the map
    // overwriting the old rules where necessary

    // These rules
    // 8: 42 | 42 8
    // 11: 42 31 | 42 11 31

    // are equivalent to
    // 8: 42 (8 | ε)
    // 11: 42 (11 | ε) 31

    // but we don't have brackets so turn them into their own rules
    //   8: 42 -8
    //  -8: 8 | ε
    //  11: 42 -11 31
    // -11: 11 | ε

    let rule_8 = Rule {
        number: 8,
        sequences: vec![vec![Step::SubRule(42), Step::SubRule(-8)]],
    };
    let rule_negative_8 = Rule {
        number: -8,
        sequences: vec![vec![Step::SubRule(8)], vec![Step::Epsilon]],
    };
    let rule_11 = Rule {
        number: 11,
        sequences: vec![vec![Step::SubRule(42), Step::SubRule(-11), Step::SubRule(31)]],
    };
    let rule_negative_11 = Rule {
        number: -11,
        sequences: vec![vec![Step::SubRule(11)], vec![Step::Epsilon]],
    };

    rules_map.insert(8, rule_8);
    rules_map.insert(-8, rule_negative_8);
    rules_map.insert(11, rule_11);
    rules_map.insert(-11, rule_negative_11);

    return messages_str
        .lines()
        .filter(|message| {
            let mut s = message.chars();

            let rule_passes = rules_map.get(&0).unwrap()
                .pass(&mut s, &rules_map);

            // rule passes and there's no more characters that need checking
            return rule_passes && s.count() == 0;
        })
        .count() as i64;
}

enum Step {
    SubRule(i64),
    Char(char),
    Epsilon,
}

impl Step {
    pub fn parse(s: &str) -> Step {
        // parse int
        // success: Symbol
        // fail: take 2nd char from string, return Char

        match s.parse::<i64>() {
            Ok(num) => Step::SubRule(num),
            Err(_) => {
                // assume it's a char
                Step::Char(s.chars().nth(1).unwrap())
            }
        }
    }

    pub fn pass(&self, s: &mut Chars, rules: &HashMap<i64,Rule>) -> bool {
        // if I want a char, check the char
        // else call pass on the SubRule referenced
        match *self {
            Step::Char(expected_char) => {
                match s.next() {
                    Some(actual_char) => expected_char == actual_char,
                    None => false,
                }
            },
            Step::SubRule(num) => {
                return rules.get(&num).unwrap().pass(s, rules);
            },
            Step::Epsilon => true,
        }
    }
}

type Sequence = Vec<Step>;

fn sequence_parse(s: &str) -> Sequence {
    // split each subrule string by ' ' to get a list of step strings
    // parse string into step
    s.split(' ').map(Step::parse).collect::<Sequence>()
}

fn sequence_pass(sequence: &Sequence, s: &mut Chars, rules: &HashMap<i64,Rule>) -> bool {
    // return true if all steps pass

    for step in sequence {
        if !step.pass(s, rules) {
            return false;
        }
    }

    return true;
}

struct Rule {
    pub number: i64,
    pub sequences: Vec<Sequence>,
}

impl Rule {
    pub fn parse(s: &str) -> Rule {
        // split by ": "
        let mut itr = s.split(": ");

        // left is number
        let number_str = itr.next().unwrap();

        // right split by | to get a list of subrules strings
        let sequences_strs = itr.next().unwrap().split(" | ");

        Rule {
            number: number_str.parse::<i64>().unwrap(),
            sequences: sequences_strs.map(sequence_parse).collect::<Vec<Sequence>>(),
        }
    }

    pub fn pass(&self, s: &mut Chars, rules: &HashMap<i64,Rule>) -> bool {
        // return true if any sub rule passes

        for seq in &self.sequences {
            let mut testing_s = s.clone();
            if sequence_pass(&seq, &mut testing_s, rules) {
                // continue where we left off
                sequence_pass(&seq, s, rules); // slow :(
                return true;
            }
        }

        return false;
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1("0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb
"), 2);
    }

    #[test]
    fn basic_1() {
        assert_eq!(part1("0: \"a\"

a
"), 1);
    }

    #[test]
    fn basic_2() {
        assert_eq!(part1("0: \"a\"

b
"), 0);
    }

    #[test]
    fn basic_3() {
        assert_eq!(part1("0: \"a\"

aa
"), 0);
    }
}
