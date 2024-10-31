#[derive(Clone, Copy, Debug)]
struct Fruit<T> {
    calories: i32,
    data: T,
}

impl <T> Fruit<T> {
    fn new(calories: i32, data: T) -> Fruit<T> {
        Self{
            calories,
            data
        }
    }

    // can be used to clone the Fruit struct if it does not already implement the Clone trait, in
    // this case it does, but if not, the below can be commented out
    // fn clone(&self) -> Self<T> {
    //     Self::new(
    //         self.calories.clone(),
    //         self.title.clone(),
    //         self.data.clone()
    //     )
    // }
}
