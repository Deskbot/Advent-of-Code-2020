use std::collections::HashMap;
use std::fs;

pub fn day19() {
    let file = fs::read_to_string("input/day19.txt").expect("input not found");

    println!("Part 1: {}", part1(&file));
    // println!("Part 2: {}", part2(&file));
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
        .filter(|message| rules_map.get(&0).unwrap().pass(&message, &rules_map))
        .count() as i64;
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

        match s.parse::<i64>() {
            Ok(num) => Step::SubRule(num),
            Err(_) => {
                // assume it's a char
                Step::Char(s.chars().nth(1).unwrap())
            }
        }
    }

    pub fn pass(&self, s: &str, rules: &HashMap<i64,Rule>) -> bool {
        // if I want a char, check the char
        // else call pass on the SubRule referenced
        match *self {
            Step::Char(c) => c == s.chars().next().unwrap(),
            Step::SubRule(num) => rules.get(&num).unwrap().pass(s, rules),
        }
    }
}

type Sequence = Vec<Step>;

fn sequence_parse(s: &str) -> Sequence {
    // split each subrule string by ' ' to get a list of step strings
    // parse string into step
    s.split(' ').map(Step::parse).collect::<Sequence>()
}

fn sequence_pass(sequence: &Sequence, s: &mut &str, rules: &HashMap<i64,Rule>) -> bool {
    // return true if all steps pass

    for step in sequence {
        if step.pass(s, rules) {
            // inc str
            let rest_of_str = &s[1..];
            *s = rest_of_str;
        } else {
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

    pub fn pass(&self, s: &str, rules: &HashMap<i64,Rule>) -> bool {
        // return true if any sub rule passes

        let mut ref_str = s;

        return self.sequences.iter()
            .any(|seq| sequence_pass(seq, &mut ref_str, rules))
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
}
