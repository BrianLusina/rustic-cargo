pub trait Operations {
    fn double(&self) -> Self;

    fn square(&self) -> Self;
}

impl Operations for i32 {
    fn double(&self) -> Self {
        self * 2
    }

    fn square(&self) -> Self {
        self * self
    }
}

impl Operations for i64 {
    fn double(&self) -> Self {
        self * 2
    }

    fn square(&self) -> Self {
        self * self
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::primitives::Operations;

    #[test]
    fn double() {
        let x: i32 = 5;
        let y = 5_i64;
        assert_eq!(x.double(), 10);
        assert_eq!(y.double(), 10);
    }

    #[test]
    fn square_i32() {
        let x: i32 = 5;
        let y: i64 = 5_i64;
        assert_eq!(x.square(), 25);
        assert_eq!(y.square(), 25);
    }
}