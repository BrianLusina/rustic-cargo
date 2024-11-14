mod front_of_house;
mod back_of_house;

use crate::front_of_house::hosting;
// the below is also correct and is valid, however, the above is a better and more idiomatic way of
// showing what is in scope and makes it clear that we are using the add_to_waitlist function from
// the hosting module, prefer the above use of 'use' over the below
// use crate::front_of_house::hosting::add_to_waitlist;

fn deliver_order() {}

pub fn eat_at_restaurant() {
    // absolute path
    // crate::front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    // relative path
    // front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    // order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::breakfast::Breakfast::summer("Rye");

    // change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // this will not compile as seasonal_fruit is a private field in the public struct Breakfast
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::appetizer::Appetizer::Soup;
    let order2 = back_of_house::appetizer::Appetizer::Salad;
}