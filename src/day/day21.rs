use std::fs;

use regex::Regex;

pub fn day21() {
    let file = fs::read_to_string("input/day21.txt").expect("input not found");

    println!("Part 1: {}", part1(&file));
    // println!("Part 2: {}", part2(&file));
}

fn part1(input: &str) -> i64 {
    input.lines()
        .map(Food::parse)
        .for_each(|a| println!("{:?}", a));
    0
}

#[derive(Debug)]
struct Food<'a> {
    allergens: Vec<&'a str>,
    ingredients: Vec<&'a str>,
}

impl<'a> Food<'a> {
    pub fn parse(s: &str) -> Food {
        let mut itr = s.split(" (contains ");
        let ingredients_str = itr.next().unwrap();
        let allergens_str = itr.next().unwrap();

        let allergens = allergens_str.split(")").next().unwrap()
            .split(" ")
            .collect();
        let ingredients = ingredients_str
            .split(" ")
            .collect();

        return Food {
            allergens,
            ingredients,
        };
    }
}
