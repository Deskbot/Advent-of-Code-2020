
use crate::util::{
    both,
    option_bind,
};
struct Rule {
    allowed_index: Vec<i32>,
    letter: char,
}

impl Rule {
    pub fn test(&self, password: &str) -> bool {

        let chars_in_pos = self.allowed_index.iter().filter(|&elf_index| {
            let mut itr = password.chars();

            let index = elf_index - 1;
            let char_at_index = itr.nth(index as usize);

            return char_at_index.is_some() && self.letter == char_at_index.unwrap();
        });

        return chars_in_pos.count() == 1;
    }
}

pub fn test_line(line: &str) -> bool {
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
            allowed_index: parse_pair(range),
            letter,
        })
    })
    .expect(format!("Couldn't parse rule: {}", rule).as_str())
}

fn parse_pair(range: &str) -> Vec<i32> {
    let mut itr = range.split("-");
    let left  = option_bind(itr.next(), |num| num.parse::<i32>().ok());
    let right = option_bind(itr.next(), |num| num.parse::<i32>().ok());

    both(left, right).and_then(|(left,right)| {
        Some(vec![left,right])
    })
    .expect(format!("Couldn't parse pair: {}", range).as_str())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert!(!test_line("1-3 b: cdefg"));
        assert!(!test_line("2-9 c: ccccccccc"));
        assert!(test_line("1-3 a: abcde"));
    }
}
