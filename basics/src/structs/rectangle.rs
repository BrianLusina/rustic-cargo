#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn set_width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    fn set_height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_creation() {
        let rectangle = Rectangle::new(10, 20);
        assert_eq!(rectangle.width, 10);
        assert_eq!(rectangle.height, 20);
    }

    #[test]
    fn test_area() {
        let rectangle = Rectangle::new(10, 20);
        assert_eq!(rectangle.area(), 200);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let rectangle_one = Rectangle::new(30, 50);
        let rect_two = Rectangle::new(10, 40);
        assert!(rectangle_one.can_hold(&rect_two));

        let rect_three = Rectangle::new(60, 45);
        assert!(!rectangle_one.can_hold(&rect_three));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger_rectangle = Rectangle::new(30, 50);
        let smaller_rectangle = Rectangle::new(10, 40);
        assert!(!smaller_rectangle.can_hold(&larger_rectangle));
    }
}
