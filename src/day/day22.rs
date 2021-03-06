use std::{collections::VecDeque, fs};

pub fn day22() {
    let file = fs::read_to_string("input/day22.txt").expect("input not found");

    let (p1,p2) = parse_input(&file);

    println!("Part 1: {}", part1(p1.clone(), p2.clone()));
    println!("Part 2: {}", part2(p1, p2));
}

fn parse_input(input: &str) -> (Deck, Deck) {
    let mut itr = input.split("\n\n");
    let p1_input = itr.next().unwrap();
    let p2_input = itr.next().unwrap();

    return (Deck::parse(p1_input),
            Deck::parse(p2_input));
}

fn part1(mut p1: Deck, mut p2: Deck) -> i64 {
    let winner = loop {
        if p1.is_empty() {
            break &p2;
        }
        if p2.is_empty() {
            break &p1;
        }

        let card_1 = p1.draw().unwrap();
        let card_2 = p2.draw().unwrap();

        if card_1 > card_2 {
            p1.append(card_1);
            p1.append(card_2);
        } else {
            p2.append(card_2);
            p2.append(card_1);
        }
    };

    return winner.score();
}

fn part2(p1: Deck, p2: Deck) -> i64 {
    0
}

#[derive(Clone, Debug)]
struct Deck(VecDeque<i64>);

impl Deck {
    pub fn parse(s: &str) -> Deck {
        let q = s.lines()
            .map(str::parse::<i64>)
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .collect();

        return Deck(q);
    }

    pub fn append(&mut self, card: i64) {
        self.0.push_back(card);
    }

    pub fn draw(&mut self) -> Option<i64> {
        return self.0.pop_front();
    }

    pub fn is_empty(&self) -> bool {
        return self.0.len() == 0;
    }

    pub fn score(&self) -> i64 {
        let mut multiplier = self.0.len() as i64;
        let mut total = 0;

        for card in &self.0 {
            total += card * multiplier;
            multiplier -= 1;
        }

        return total;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10
";

    #[test]
    fn part1_example() {
        let (p1,p2) = parse_input(EXAMPLE_1); // I shouldn't have to do this.
        assert_eq!(part1(p1,p2), 306);
    }

    #[test]
    fn part1_actual() {
        let file = fs::read_to_string("input/day22.txt").expect("input not found");
        let (p1,p2) = parse_input(&file);
        assert_eq!(part1(p1,p2), 31957);
    }
}
