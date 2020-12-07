use std::fs;

struct Bag<'a> {
    contains: Vec<&'a str>,
}

static mut last_colour_id: i32 = 0;

impl Bag<'_> {
    pub fn new<'a>(bags: Vec<&'a str>) -> Bag<'a> {
        Bag { contains: bags }
    }

    pub fn contains(bag: &Bag) -> bool {}
}

pub fn day06() {
    let file = fs::read_to_string("input/day07.txt").expect("input not found");

    println!("Part 1: {}", part1(&file));
    // println!("Part 2: {}", part2(&file));
}

fn part1(input: &str) -> i32 {
    // split by line
    // split by " bags contain "
    // left is holding bag
    // right is bags held str
    // split right by ", " to get a list of numbers of bags
    // then split those strings by the first space
    // left is the number of bags, right is the held bag colour (str ends with bag or bags)
}

// fn part2(input: &str) -> i32 {

// }
