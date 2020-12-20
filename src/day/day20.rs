use std::str::Chars;
use std::collections::HashMap;
use std::fs;

pub fn day20() {
    let file = fs::read_to_string("input/day20.txt").expect("input not found");

    println!("Part 1: {}", part1(&file));
    // println!("Part 2: {}", part2(&file));
}

fn part1(input: &str) -> i64 {
    // split into tile strings

    // parse tiles
    // into struct with tile num and 2d grid
    // add tile method for getting each edge

    // for each tile (my_tile)
    // for each other tile
    // count how many of (my_tile)'s edges are unique

    // *= tiles that have exactly 2 unique edges
    0
}
