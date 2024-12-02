use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// simpler version of the trait bound syntax. This is known as the impl trait bound
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize())
}

// longer form of the above trait bound
fn print<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize())
}

// multiple trait bound syntax using the + operator. This means that the item passed in must
// implement both the Summary Trait and Display Trait.
fn notify2(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize())
}

// multiple trait bound syntax using the + operator on generic types
fn notify3<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize())
}

// multiple trait bounds using the + operator on the generic types.
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    30
}

// multiple trait bounds using the where clause operator on the generic types. This makes it more
// readable and expressive
fn some_function2<T,U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug
{
    30
}

// this specifies that this function returns some type that implements the Summary trait. This
// makes it such that functions do not have to specify the concret type in the return position of a
// function. This also means that the calling function doesn't need to know the exact concrete type
// returned, just a type that implements the trait.
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("random_user"),
        content: String::from("Random content"),
        reply: false,
        retweet: false,
    }
}

#[cfg(test)]
mod summary_tests {
    use crate::traits::summary::Summary;
    use crate::traits::summary::Tweet;
    use crate::traits::summary::NewsArticle;

    #[test]
    fn summarizes_tweets() {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        };
        let expected = "horse_ebooks: of course, as you probably already know, people";
        assert_eq!(tweet.summarize(), expected);
    }
}