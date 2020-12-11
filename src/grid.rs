pub struct Grid<T> {
    grid: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn new(vec_2d: Vec<Vec<T>>) -> Grid<T> {
        Grid {
            grid: vec_2d,
        }
    }

    pub fn fmap<U>(&self, mapper: fn(&T) -> U) -> Grid<U> {
        Grid {
            grid:
                self.grid.iter()
                    .map(|row| row.iter()
                        .map(mapper)
                        .collect::<Vec<U>>()
                    )
                    .collect::<Vec<Vec<U>>>()
        }
    }
}
