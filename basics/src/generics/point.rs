struct Point<X, Y> {
    x: X,
    y: Y,
}

impl<X, Y> Point<X, Y> {
    fn x(&self) -> &X {
        &self.x
    }

    fn y(&self) -> &Y {
        &self.y
    }

    fn new(x: X, y: Y) -> Point<X, Y> {
        Point { x: x, y: y }
    }

    fn mixup(self, other: Point<X, Y>) -> Point<X, Y> {
        Self {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
