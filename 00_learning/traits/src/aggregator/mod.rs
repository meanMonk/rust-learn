// pub trait Summary {
//     fn summarize(&self) -> String;
// }

// Default Implementations
// It's useful to have default behaviour for some or all of the methods in trait instead of requiring implementations for all methods on every type.

pub trait Summary {
    
    fn summarize_author(&self) -> Self;
    
    fn summarize(&self) -> String {
        String::from("(Read more..)")
    }
    
    
    
};

// to use default implementation of methods we called it like 

pub struct LinkedinPost {
    pub content: String,
    pub author: String,
    pub likes : i32,
}

impl Summary for LinkedinPost {}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, {} by {} ({})", self.headline,self.content, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {} - reply {} retweet {} ", self.username, self.content, self.reply, self.retweet)
    }
}


// Traits as parameters. 

pub fn notify(item: &impl Summary) {
    println!("Breaking news {}", item.summarize());
}

// Traits bound syntax

pub fn bound_notify<T: Summary>(item: &T) {
    println!("Breaking news {}", item.summarize());
}

// Multiple traits bound syntax.

pub fn multi_bound_notify<T: Summary + Display> (item: &T) {
    
}

// clearer trait bounds with where clauses.

fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32{
    
}

// with use of where clauses.

fn some_function<T, U>(t: &T, u:&U) -> i32 
where 
    T: Display + Clone, 
    U: Clone + Debug,

{
    
}

// Returning types that implements traits..

fn returning_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}