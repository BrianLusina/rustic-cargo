fn print_square(row_number: i32) -> () {
    let mut row = 0;
    while row < row_number {
        let mut col = 0;
        while col < 10 {
            print!("{}", if row == col { "X" } else { "O" });
            col += 1;
        }
        println!();
        row += 1;
    }
}

fn draw_triangle(n: i32) -> () {
    let mut row = 1;
    while row <= n {
        let mut col = 1;
        while col <= row {
            print!("X");
            col += 1;
        }
        println!();
        row += 1;
    }
}
