struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn set_x(mut self, x: i32) -> Self {
        self.x = x;
        self
    }

    fn set_y(mut self, x: i32) -> Self {
        self.x = x;
        self
    }
}
