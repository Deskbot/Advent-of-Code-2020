use crate::{day::day11::Seat, point::Point};
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

pub struct Grid<T: Display> {
    cols: usize,
    grid: Vec<Vec<T>>,
    rows: usize,
}

impl<T: Eq + Clone + Copy + Debug + Display> Grid<T> {
    pub fn new(vec_2d: Vec<Vec<T>>) -> Grid<T> {
        let first_row_len = vec_2d[0].len();

        let all_rows_same_len =
            vec_2d.iter()
                .all(|row| row.len() == first_row_len);

        assert!(all_rows_same_len);

        Grid {
            cols: first_row_len,
            rows: vec_2d.len(),
            grid: vec_2d,
        }
    }

    pub fn fmap<U: Display>(&self, mapper: fn(&T) -> U) -> Grid<U> {
        Grid {
            cols: self.cols,
            grid:
                self.grid.iter()
                    .map(|row| row.iter()
                        .map(mapper)
                        .collect::<Vec<U>>()
                    )
                    .collect::<Vec<Vec<U>>>(),
            rows: self.rows,
        }
    }

    fn coord_map<U: Display>(&self, get_new_cell_value: impl Fn((usize,usize)) -> U) -> Grid<U> {
        let mut grid = Vec::<Vec<U>>::with_capacity(self.rows);

        for row_num in 0..self.grid.len() {
            grid.insert(row_num, Vec::with_capacity(self.cols));

            let &row = &self.grid.get(row_num).unwrap();
            for col_num in 0..row.len() {
                let new_cell = get_new_cell_value((row_num, col_num));
                grid.get_mut(row_num).unwrap().insert(col_num, new_cell);
            }
        }

        Grid {
            cols: self.cols,
            grid,
            rows: self.rows,
        }
    }

    pub fn eq(&self, them: &Grid<T>) -> bool {
        for (&mine, theirs) in self.cell_iter().iter().zip(them.cell_iter()) {
            if mine != theirs {
                return false;
            }
        }

        return true;
    }

    pub fn game_of_life(&self, new_cell_value: fn (was: &T, neighbours: &Vec<&T>) -> T) -> Grid<T> {
        self.coord_map(|(row_num, col_num)| {
            let neighbours = self.get_neighbours(row_num, col_num);
            return new_cell_value(self.get(row_num, col_num), &neighbours);
        })
    }

    pub fn get(&self, row_num: usize, col_num: usize) -> &T {
        return self.grid.get(row_num).unwrap().get(col_num).unwrap();
    }

    fn get_neighbours(&self, row_num: usize, col_num: usize) -> Vec<&T> {
        let mut result = Vec::with_capacity(8);

        let mut offsets: Vec<(i32, i32)> = vec![
            (-1, -1),
            (-1,  0),
            (-1,  1),
            ( 0, -1),
            ( 0,  1),
            ( 1, -1),
            ( 1,  0),
            ( 1,  1),
        ];

        if row_num == 0 {
            offsets = offsets.into_iter().filter(|&(r,_)| r != -1).collect();
        }

        if row_num == self.rows - 1 {
            offsets = offsets.into_iter().filter(|&(r,_)| r != 1).collect();
        }

        if col_num == 0 {
            offsets = offsets.into_iter().filter(|&(_,c)| c != -1).collect();
        }

        if col_num == self.cols - 1 {
            offsets = offsets.into_iter().filter(|&(_,c)| c != 1).collect();
        }

        for (r_off, c_off) in offsets {
            let r = (row_num as i32 + r_off) as usize;
            let c = (col_num as i32 + c_off) as usize;

            result.push(&self.grid[r][c]);
        }

        return result;
    }

    pub fn cell_iter(&self) -> Vec<T> {
        let mut result = Vec::with_capacity(self.rows * self.cols);

        for row in &self.grid {
            result.append(&mut row.clone());
        }

        return result;
    }
}

impl Grid<Seat> {
    pub fn game_of_life_part_2(&self, new_cell_value: fn (was: &Seat, visible: &Vec<&Seat>) -> Seat) -> Grid<Seat> {
        self.coord_map(|(row_num, col_num)| {
            let visible = self.get_visible(row_num, col_num);
            return new_cell_value(self.get(row_num, col_num), &visible);
        })
    }

    fn get_visible(&self, row_num: usize, col_num: usize) -> Vec<&Seat> {
        let mut visible = Vec::with_capacity(self.cols + self.rows);

        let directions: Vec<Point> = vec![
            Point::new(-1, -1),
            Point::new(-1,  0),
            Point::new(-1,  1),
            Point::new( 0, -1),
            Point::new( 0,  1),
            Point::new( 1, -1),
            Point::new( 1,  0),
            Point::new( 1,  1),
        ];

        // row is x, col is y because it's easier to think about
        let from = Point::new(row_num as i32, col_num as i32);

        for direction in directions {
            for times in 1.. {
                let might_see_pos = direction.multiply(times).plus(&from);

                if !self.pos_exists(might_see_pos.x, might_see_pos.y) {
                    break;
                }

                let cell = self.get(might_see_pos.x as usize, might_see_pos.y as usize);
                visible.push(cell);

                if *cell != Seat::Floor {
                    break;
                }
            }
        }

        return visible;
    }

    fn pos_exists(&self, row_num: i32, col_num: i32) -> bool {
        row_num >= 0 && col_num >= 0
            && row_num < self.rows as i32
            && col_num < self.cols as i32
    }
}

impl<T: Eq + Clone + Copy + Debug + Display> Display for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for row in &self.grid {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            write!(f, "\n")?;
        }

        Result::Ok(())
    }
}
