use std::{collections::VecDeque, fs};

pub fn day22() {
    let file = fs::read_to_string("input/day22.txt").expect("input not found");

    println!("Part 1: {}", part1(&file));
    // println!("Part 2: {}", part2(&file));
}

fn part1(input: &str) -> i64 {
    // parse input
    let mut itr = input.split("\n\n");
    let p1_input = itr.next().unwrap();
    let p2_input = itr.next().unwrap();

    let mut p1 = Deck::parse(p1_input);
    let mut p2 = Deck::parse(p2_input);

    let winner = loop {
        let card_1 = match p1.draw() {
            Some(card_1) => card_1,
            None => break p2,
        };
        let card_2 = match p2.draw() {
            Some(card_2) => card_2,
            None => break p1,
        };

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
