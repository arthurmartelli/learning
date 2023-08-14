#![allow(unused)]

use crate::{
    aggregator::Summary,
    media::{notify, NewsArticle, Tweet},
};

mod aggregator {
    pub trait Summary {
        fn alert_message(&self) -> &str;
        fn summarize_author(&self) -> String;
        fn summarize(&self) -> String {
            format!("(Read from {})", self.summarize_author())
        }
    }
}

mod media {
    use crate::aggregator::Summary;

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn alert_message(&self) -> &str {
            return "Breaking News!";
        }

        fn summarize_author(&self) -> String {
            return self.author.clone();
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn alert_message(&self) -> &str {
            return "New Tweet!";
        }
        fn summarize_author(&self) -> String {
            return format!("@{}", self.username);
        }
    }

    pub fn notify(item: &impl Summary) -> () {
        println!("{} {}", item.alert_message(), item.summarize())
    }
}

mod pair {
    use std::fmt::Display;

    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) -> String {
            if self.x >= self.y {
                return format!("The largest member is x = {}", self.x);
            } else {
                return format!("The largest member is y = {}", self.y);
            }
        }
    }
}

fn main() {
    let tweet: Tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let article: NewsArticle = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
            hockey team in the NHL.",
        ),
    };

    notify(&tweet);
    notify(&article);
}
