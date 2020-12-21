use std::{collections::{HashMap, HashSet}, fs};

pub fn day21() {
    let file = fs::read_to_string("input/day21.txt").expect("input not found");

    println!("Part 1: {}", part1(&file));
    // println!("Part 2: {}", part2(&file));
}

fn part1(input: &str) -> i64 {
    let mut allergens_to_ingredients = HashMap::<&str, HashSet<&str>>::new(); // allergen to set of possible foods that contain it

    let foods = input.lines().map(Food::parse).collect::<Vec<Food>>();

    for food in &foods {
        for allergen in &food.allergens {
            let possible_ingredients = allergens_to_ingredients
                .entry(allergen).or_insert(HashSet::new());

            let new_possible_ingredients = possible_ingredients
                .intersection(&food.ingredients)
                .map(|&a| a)
                .collect::<HashSet<&str>>();
            allergens_to_ingredients.insert(allergen, new_possible_ingredients);
        }
    }

    // unionise all the ingredients sets into one set
    let all_ingredients = foods.iter().map(|food| &food.ingredients)
        .fold(
            HashSet::new(),
            |acc, next| acc
                .union(&next)
                .map(|&s| s)
                .collect::<HashSet<&str>>()
        );

    // unionise all the ingredients sets into one set
    let all_allergenic_ingredients = allergens_to_ingredients.values()
        .fold(
            HashSet::new(),
            |acc, next| acc
                .union(next)
                .map(|&s| s)
                .collect::<HashSet<&str>>()
        );

    let non_allergenic_ingredients = all_ingredients.difference(&all_allergenic_ingredients);

    println!("{:?}", non_allergenic_ingredients);

    0
}

#[derive(Debug)]
struct Food<'a> {
    allergens: HashSet<&'a str>,
    ingredients: HashSet<&'a str>,
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
