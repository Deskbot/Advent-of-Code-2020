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

    // only 0 uses 8 and 11
    // 0: 8 11
    // 8: 42
    // 11: 42 31
    // the solution is to count how many times I can parse 42,
    // then count how many times I can parse 31
    // pass if 42 is double 31

    return messages_str
        .lines()
        .filter(|message| {
            let mut char_iter = message.chars();

            let mut forty_twos = 0;
            let mut thirty_ones = 0;

            let mut doing_42 = true;

            loop {
                let mut s = char_iter.clone();

                if doing_42 {
                    let rule_passes = rules_map.get(&42).unwrap()
                        .pass(&mut s, &rules_map);

                    if rule_passes {
                        forty_twos += 1;
                        char_iter = s;
                        continue;
                    } else {
                        doing_42 = false;
                        continue;
                    }

                } else {
                    let rule_passes = rules_map.get(&31).unwrap()
                        .pass(&mut s, &rules_map);

                    if rule_passes {
                        thirty_ones += 1;
                        char_iter = s;
                        continue;
                    } else {
                        break;
                    }
                }
            }

            // rule passes and there's no more characters that need checking
            // 42+ 42{n} 31{n}
            // there's always at least one more 42 than 31
            return forty_twos > thirty_ones
                && forty_twos > 0
                && thirty_ones > 0
                && char_iter.count() == 0;
        })
        .count() as i64;
}

#[derive(Clone)]
enum Step {
    SubRule(i64),
    Char(char),
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
            }
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

    #[test]
    fn part2_example() {
        assert_eq!(part2("42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
"), 12);
    }

    #[test]
    fn part2_example_1() {
        assert_eq!(part2("42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

bbabbbbaabaabba
"), 1);
    }

    #[test]
    fn part2_example_2() {
        assert_eq!(part2("42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

babbbbaabbbbbabbbbbbaabaaabaaa
"), 1);
    }

    #[test]
    fn part2_example_3() {
        assert_eq!(part2("42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

aaabbbbbbaaaabaababaabababbabaaabbababababaaa
"), 1);
    }

    #[test]
    fn part2_example_4() {
        assert_eq!(part2("42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

bbbbbbbaaaabbbbaaabbabaaa
"), 1);
    }

    #[test]
    fn part2_example_5() {
        assert_eq!(part2("42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

bbbababbbbaaaaaaaabbababaaababaabab
"), 1);
    }

    #[test]
    fn part2_example_6() {
        assert_eq!(part2("42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

ababaaaaaabaaab
"), 1);
    }

    #[test]
    fn part2_example_7() {
        assert_eq!(part2("42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

ababaaaaabbbaba
"), 1);
    }

    #[test]
    fn part2_example_8() {
        assert_eq!(part2("42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

baabbaaaabbaaaababbaababb
"), 1);
    }

    #[test]
    fn part2_example_9() {
        assert_eq!(part2("42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbabbbbaaaababbbbbbaaaababb
"), 1);
    }

    #[test]
    fn part2_example_10() {
        assert_eq!(part2("42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

aaaaabbaabaaaaababaa
"), 1);
    }

    #[test]
    fn part2_example_11() {
        assert_eq!(part2("42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
"), 1);
    }

    #[test]
    fn part2_example_12() {
        assert_eq!(part2("42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
"), 1);
    }


    #[test]
    fn part2_example_13() {
        assert_eq!(part2("42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
aaaabbaaaabbaaa
babaaabbbaaabaababbaabababaaab
"), 0);
    }
}
