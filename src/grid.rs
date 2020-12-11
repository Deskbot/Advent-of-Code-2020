use std::fmt::Debug;

pub struct Grid<T> {
    cols: usize,
    grid: Vec<Vec<T>>,
    rows: usize,
}

impl<T: Eq + Clone + Copy + Debug> Grid<T> {
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

    pub fn fmap<U>(&self, mapper: fn(&T) -> U) -> Grid<U> {
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

    fn coord_map<U: Debug>(&self, get_new_cell_value: impl Fn((usize,usize)) -> U) -> Grid<U> {
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

    fn get(&self, row_num: usize, col_num: usize) -> &T {
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
            offsets = offsets.into_iter().filter(|&(r,c)| r != -1).collect();
        }

        if row_num == self.rows - 1 {
            offsets = offsets.into_iter().filter(|&(r,c)| r != 1).collect();
        }

        if col_num == 0 {
            offsets = offsets.into_iter().filter(|&(r,c)| c != -1).collect();
        }

        if col_num == self.cols - 1 {
            offsets = offsets.into_iter().filter(|&(r,c)| c != 1).collect();
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
