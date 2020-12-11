pub struct Grid<T> {
    grid: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    fn new(vec_2d: Vec<Vec<T>>) -> Grid<T> {
        Grid {
            grid: vec_2d,
        }
    }
}
