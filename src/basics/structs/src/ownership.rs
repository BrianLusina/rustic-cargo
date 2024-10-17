fn ownership() {
    { // s is not valid here, as it has not yet been declared
        let mut s = String::from("Hello there"); // s is valid from this point
        // do some magic with s
        s.push_str(", I am a new Rust Bot, you can call me Nora");
        println!("{s}")
    } // scope is now over and s is no longer valid

}
