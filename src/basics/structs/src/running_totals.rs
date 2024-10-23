struct RunningTotal {
    total: i32,
    count: i32,
}

impl RunningTotal {
  fn new(total: i32, count: i32) -> Self {
    Self {
        total: total,
        count: count,
    }
  }
  
  fn print_average(&mut self) {
    if self.count == 0 {
        println!("I don't have any values yet!");
    } else {
        println!("{}", self.total / self.count);
    }
  }
  
  fn add_value(&mut self, value: i32) {
      self.total += value;
      self.count += 1;
  }
}

fn main() {
    let mut rt = RunningTotal::new(0, 0);

    rt.print_average();

    rt.add_value(5);
    rt.add_value(3);
    rt.print_average();

    rt.add_value(10);
    rt.print_average();
}