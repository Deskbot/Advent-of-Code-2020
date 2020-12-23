use std::fs;

pub fn day23() {
    println!("Part 1: {}", part1("792845136"));
}

fn part1(input: &str) -> i64 {
    let mut cups = Cups::parse(input);

   0
}

struct Cups {
    current_cup: usize,
    list: Vec<i64>
}

impl Cups {
    pub fn parse(s: &str) -> Cups {
        Cups {
            current_cup: 0,
            list: s.chars()
                .map(|ch| ch.to_digit(10))
                .map(Option::unwrap)
                .map(|d| d as i64)
                .collect(),
        }
    }

    pub fn step(&mut self)  {

    }
}
