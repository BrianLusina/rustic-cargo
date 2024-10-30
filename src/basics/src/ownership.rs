fn main() {
    let s = String::from("hello"); // s comes into scope
    let mut s1 = String::from("hello"); // s1 comes into scope

    {
        let r1 = &mut s1;
    } // r1 goes out of scope here, so we can make a new reference with no problems

    change(&mut s1);

    let len = calculate_length(&s);

    take_ownership(s); // s's value moves into the function
    // s is no longer valid here

    let x = 5; // x comes into scope

    /* x would move into the function, but i32 is Copy, so it's okay to still use x after. */
    make_copy(x);
}
/* x goes out of scope, then s, but because s's value was moved, nothing special happens */

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn ownership() {
    {
        // s is not valid here, as it has not yet been declared
        let mut s = String::from("Hello there"); // s is valid from this point
                                                 // do some magic with s
        s.push_str(", I am a new Rust Bot, you can call me Nora");
        println!("{s}")
    } // scope is now over and s is no longer valid
}

fn take_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // here, some_string goes out of scope and 'drop' is called. The backing memory is freed

fn make_copy(i: i32) {
    // i comes into scope
    println!("{i}");
} // i goes out of scope, Nothing special happens

fn calculate_length(s: &String) -> usize {
    // s is a reference to a string
    s.len()
} // s goes out of scope. But because it does not have ownership of what it refers to, it is not dropped
