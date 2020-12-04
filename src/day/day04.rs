use std::fs;

pub fn day04() {
    let file = fs::read_to_string("input/day04.txt")
        .expect("input not found");

    let trees = trees_hit(&file, &Point::new(3,1));
    println!("Part 1: {}", trees);

}
