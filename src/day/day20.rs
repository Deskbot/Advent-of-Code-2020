use std::fs;

use regex::Regex;

pub fn day20() {
    let file = fs::read_to_string("input/day20.txt").expect("input not found");

    println!("Part 1: {}", part1(&file));
    // println!("Part 2: {}", part2(&file));
}

fn part1(input: &str) -> i64 {
    // split into tile strings
    let tiles = input.split("\n\n").map(Tile::parse);

    // for each tile (my_tile)
    // for each other tile
    // count how many of (my_tile)'s edges are unique
    // *= tiles that have exactly 2 unique edges

    let mut corner_tiles = Vec::with_capacity(4);

    for tile in tiles.clone() {
        let my_left = &tile.left();
        let my_right = &tile.right();
        let my_top = tile.top();
        let my_bottom = tile.bottom();

        let mut left_matches = 0;
        let mut right_matches = 0;
        let mut top_matches = 0;
        let mut bottom_matches = 0;

        for other_tile in tiles.clone() {
            if tile == other_tile {
                continue;
            }

            let other_edges = other_tile.get_edges();

            if other_edges.iter().any(|other_edge| edges_eq(my_left, other_edge)) {
                left_matches += 1;
            }

            if other_edges.iter().any(|other_edge| edges_eq(my_right, other_edge)) {
                right_matches += 1;
            }

            if other_edges.iter().any(|other_edge| edges_eq(my_top, other_edge)) {
                top_matches += 1;
            }

            if other_edges.iter().any(|other_edge| edges_eq(my_bottom, other_edge)) {
                bottom_matches += 1;
            }
        }

        let mut number_of_unique_edges = 0;

        if left_matches > 0 {
            number_of_unique_edges += 1;
        }
        if right_matches > 0 {
            number_of_unique_edges += 1;
        }
        if top_matches > 0 {
            number_of_unique_edges += 1;
        }
        if bottom_matches > 0 {
            number_of_unique_edges += 1;
        }

        if number_of_unique_edges == 2 {
            corner_tiles.push(tile);
        }
    }

    assert_eq!(corner_tiles.len(), 4);

    return corner_tiles
        .iter()
        .map(|tile| tile.number)
        .product();
}

#[derive(Eq, PartialEq)]
struct Tile {
    pub number: i64,
    grid: Vec<Vec<char>>,
}

impl Tile {
    pub fn parse(s: &str) -> Tile {
        let mut itr = s.lines();
        let num_line = itr.next().unwrap();

        let number_regex = Regex::new(r"^Tile ([0-9]+):$").unwrap();
        let number = number_regex.captures(num_line).unwrap().get(1).unwrap()
            .as_str()
            .parse()
            .unwrap();

        let grid = itr
            .map(|line| line.chars().collect())
            .collect();

        return Tile {
            number,
            grid,
        };
    }

    pub fn get_edges(&self) -> Vec<Vec<char>> {
        vec![
            self.left(),
            self.right(),
            self.top().to_owned(),
            self.bottom().to_owned(),
        ]
    }

    pub fn left(&self) -> Vec<char> {
        let mut v = Vec::with_capacity(10);

        for row in &self.grid {
            v.push(row[0]);
        }

        return v;
    }

    pub fn right(&self) -> Vec<char> {
        let mut v = Vec::with_capacity(10);

        let last_index = self.grid.len() - 1;

        for row in &self.grid {
            v.push(row[last_index]);
        }

        return v;

    }

    pub fn top(&self) -> &Vec<char> {
        &self.grid[0]
    }

    pub fn bottom(&self) -> &Vec<char> {
        &self.grid[self.grid.len() - 1]
    }
}

fn edges_eq(edge1: &Vec<char>, edge2: &Vec<char>) -> bool {
    if edge1 == edge2 {
        return true;
    }

    let mut edge1_reversed = edge1.clone();
    edge1_reversed.reverse();

    return &edge1_reversed == edge2;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1("Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...
"), 20899048083289);
    }
}