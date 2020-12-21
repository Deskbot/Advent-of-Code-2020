use std::{collections::{HashMap, HashSet}, fs};

pub fn day21() {
    let file = fs::read_to_string("input/day21.txt").expect("input not found");

    println!("Part 1: {}", part1(&file));
    println!("Part 2: {}", part2(&file));
}

fn part1(input: &str) -> i64 {
    let foods = input.lines().map(Food::parse).collect::<Vec<Food>>();

    // unionise all the ingredients sets into one set
    let all_ingredients = foods.iter().map(|food| &food.ingredients)
        .fold(
            HashSet::new(),
            |acc, next| acc
                .union(&next)
                .map(|&s| s)
                .collect::<HashSet<&str>>()
        );

    // println!("{:?}", all_ingredients);

    // allergen to set of possible foods that contain it
    let mut allergens_to_ingredients = HashMap::<&str, HashSet<&str>>::new();

    for food in &foods {
        for allergen in &food.allergens {
            let possible_ingredients = allergens_to_ingredients
                .entry(allergen)
                .or_insert(all_ingredients.clone()); // before we filter down the possibilities, it could be any ingredient

            let new_possible_ingredients = possible_ingredients
                .intersection(&food.ingredients)
                .map(|&a| a)
                .collect::<HashSet<&str>>();
            allergens_to_ingredients.insert(allergen, new_possible_ingredients);
        }
    }

    // println!("{:?}", allergens_to_ingredients);

    // unionise all the ingredients sets into one set
    // all /possibly/ allergenic ingredients
    let all_allergenic_ingredients = allergens_to_ingredients.values()
        .fold(
            HashSet::new(),
            |acc, next| acc
                .union(next)
                .map(|&s| s)
                .collect::<HashSet<&str>>()
        );

    // println!("{:?}", all_allergenic_ingredients);

    // not possibly allergenic ingredients
    let non_allergenic_ingredients = all_ingredients
        .difference(&all_allergenic_ingredients)
        .map(|&s| s)
        .collect::<HashSet<&str>>();

    // println!("{:?}", non_allergenic_ingredients);

    return foods.iter()
        .map(|food| &food.ingredients)
        .flatten()
        .filter(|&&ingredient| non_allergenic_ingredients.contains(ingredient))
        .count() as i64;
}

fn part2(input: &str) -> String {
    let foods = input.lines().map(Food::parse).collect::<Vec<Food>>();

    // unionise all the ingredients sets into one set
    let all_ingredients = foods.iter().map(|food| &food.ingredients)
        .fold(
            HashSet::new(),
            |acc, next| acc
                .union(&next)
                .map(|&s| s)
                .collect::<HashSet<&str>>()
        );

    // allergen to set of possible foods that contain it
    let mut allergens_to_ingredients = HashMap::<&str, HashSet<&str>>::new();

    for food in &foods {
        for allergen in &food.allergens {
            let possible_ingredients = allergens_to_ingredients
                .entry(allergen)
                .or_insert(all_ingredients.clone()); // before we filter down the possibilities, it could be any ingredient

            let new_possible_ingredients = possible_ingredients
                .intersection(&food.ingredients)
                .map(|&a| a)
                .collect::<HashSet<&str>>();
            allergens_to_ingredients.insert(allergen, new_possible_ingredients);
        }
    }

    let mut bad_ingredients = Vec::<&str>::new();

    while allergens_to_ingredients.len() > 0  {
        let newly_known = allergens_to_ingredients
            .iter()
            .filter(|(_, ingredients)| ingredients.len() == 1)
            .map(|(&allergen,_)| allergen)
            .collect::<Vec<&str>>();

        // println!("{:?}", newly_known);

        for allergen in newly_known {
            let ingredients = allergens_to_ingredients.get(allergen).unwrap();
            // there's only 1 ingredient in this set
            let &bad_ingredient = ingredients.iter().next().unwrap();
            bad_ingredients.push(bad_ingredient);

            // remove this possibility from the other allergens
            for ing in &mut allergens_to_ingredients.values_mut() {
                ing.remove(bad_ingredient);
            }

            allergens_to_ingredients.remove(allergen);
        }
    }

    bad_ingredients.sort();
    return bad_ingredients.join(",");
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
            .split(", ")
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


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 5);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), "mxmxvkd,sqjhc,fvjkl");
    }
}
