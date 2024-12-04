// This shows how lifetimes can be used on structs. Here a lifetime is specified as a generic on the
// struct and included as well on the fields of the struct. This means that the struct will not
// outlive the field 'part'
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// lifetimes on impl of structs have to be added after the impl keyword and also after the structs
// name as they are a type for the struct
impl<'a> ImportantExcerpt<'a> {

    // Following the lifetime elision rules, this does not require the lifetime to be included in the
    // return type nor the first argument to the method. Since self is already annotated with a
    // lifetime and the return type does not require a lifetime as it is not a reference to any
    // provided parameter of the function
    fn level(&self) -> i32 {
        5
    }

    //! Using the lifetime elision rules, the self has already a lifetime attached to it and since the
    //! second argument to the function is a reference to a string slice and the return type is a
    //! reference to a string type, the lifetimes are annotated on the self and the announcement
    //! parameters. And because one of the arguments is &self, the return type gets the lifetime of
    //! &self and all lifetimes have now been accounted for, so the compiler will result in creating:
    //! ```
    //! fn announce_and_return_part<'a,'b>(&'a self, announcement: &'b str) -> &'a str {}
    //! ```
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

#[cfg(test)]
mod except_tests {
    use crate::lifetimes::excerpt::ImportantExcerpt;

    // test to show that the struct ImportantExcerpt will not outlive the 'part' field that is
    // borrowed and owned by the novel string. Therefore, this is valid and the struct will go out
    // of scope safely as the novel string is still in scope
    #[test]
    fn t_lifetimes_excerpt() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().unwrap();
        let i = ImportantExcerpt {
            part: first_sentence,
        };
        assert_eq!("Call me Ishmael", i.part);
    }
}