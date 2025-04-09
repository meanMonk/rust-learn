// Traits defines the functionality a particular type has and can share with other types. 
// we can use traits to defin shared behaviour in an abstract way.
// Note: Traits are similar to a feature called interfaces in other languages.

use aggregator::{Tweet, NewsArticle, Summary};

mod aggregator;

fn main() {
    println!("Hello, world!");
    
    let news = NewsArticle {
        headline: String::from("Content for rust goes on!"),
        location: String::from("Online!"),
        author:  String::from("rust_learner"),
        content: String::from("Rust is awseome, so start learning it from slow rustings"),
    };
    
    let tweet = Tweet {
        username: String::from("user_horse"),
        content: String::from("Content goes on rust developing step by steps!"),
        reply: false,
        retweet: false,
    };
    
    
    println!("1 new tweet: {}", tweet.summarize());
    println!("1 news articles: {}", news.summarize());
}
