fn say_apples(apples: i32) {
    println!("I can use {} apples here", apples);
    {
        // inner block with its own scope which inherits from the parent scope in say_apples. Note that
        // each function is a block with a defined scope. The arguments(parameters) of the function
        // are part of the parent scope.
        let bananas = 20;
        println!("I can use {} apples here too", apples);
        println!("I can use {} bananas here", bananas);
    }
    // bananas variables can not be used here as it is only available in the above scope, but not the
    // parent scope.
    println!("I can NOT use {} bananas here", apples);
    println!("I have {} applies", apples);
}

fn say_mangoes() {
    let mangoes = 10; // mangoes is 10
    println!("mangoes == {}", mangoes);
    {
        let mangoes = 20; // shadow mangoes in this scope
        println!("mangoes == {}", mangoes);
    }
    // That block's scope is done
    // Now our original apples is back in scope
    // What do you think this will output?
    println!("mangoes == {}", mangoes);

    /** output
    mangoes == 10
    mangoes == 20
    mangoes == 10
    **/
}

fn double(number: i32) -> i32 {
    // No need to add a ';' at the end of this expression. Functions evaluate and return the last
    // expression. If a ';' is added, it is read like a statement and will not be returned
    number * 2
}

/**
* Returns a unit -> ()
**/
fn eat(count: i32, food: &str) -> () {
    println!("You ate {} helpings of {}", count, food)
}

fn main() {
    say_apples(10);
    double(12);
    eat(10, "apples")
}
