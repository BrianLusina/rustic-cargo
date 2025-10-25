struct Point(i32, i32, i32);
struct Color(i32, i32, i32);

fn main() {
    let origin = Point(0, 0, 0);
    let red = Color(255, 0, 0);

    println!("Origin is {}, {}, {}", origin.0, origin.1, origin.2);
    println!("Color red is {}, {}, {}", red.0, red.1, red.2);
}
