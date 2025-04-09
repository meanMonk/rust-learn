// Traits defines the functionality a particular type has and can share with other types. 
// we can use traits to defin shared behaviour in an abstract way.
// Note: Traits are similar to a feature called interfaces in other languages.

// make sure to bring the trait summary in scope.

/* 
But we can’t implement external traits on external types. 
For example, we can’t implement the Display trait on Vec<T> within 
our aggregator crate because Display and Vec<T> are both defined
 in the standard library and aren’t local to our aggregator crate. 
 This restriction is part of a property called coherence, 
 and more specifically the orphan rule, so named because 
 the parent type is not present. This rule ensures that other people’s code can’t break your code and vice versa. Without the rule, two crates could implement the same trait for the same type, and Rust wouldn’t know which implementation to use.
*/




use aggregator::{Tweet, NewsArticle, Summary, LinkedinPost};


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
    
    let post = LinkedinPost {
        content: String::from("rust is awesome as of  now!"),
        author: String::from("rust_author"),
        likes: 24,
    };
    
    
    println!("1 linkedin post: {}", post.summarize());
    println!("1 new tweet: {}", tweet.summarize());
    println!("1 news articles: {}", news.summarize());
}
