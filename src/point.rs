pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point {x,y}
    }

    pub fn multiply(&self, n: i32) -> Point {
        Point {
            x: self.x * n,
            y: self.y * n,
        }
    }

    pub fn plus(&self, p: &Point) -> Point {
        Point {
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }

}