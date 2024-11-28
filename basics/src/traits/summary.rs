pub trait Summary {
    fn summarize(&self) -> String;
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