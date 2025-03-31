// # Traits

// A trait is a collection of methods.

// Data types can implement traits. To do so, the methods making up the trait are defined for the data type. For example, the `String` data type implements the `From<&str>` trait. This allows a user to write `String::from("hello")`.

// In this way, traits are somewhat similar to intefaces in other languages.

// Some additional common Rust traits include:

// - `Clone` (the `clone` method)
// - `Display` (which allows formatted display via `{}`)
// - `Debug` (which allows formatted display via `{:?}`)

// we want to make crate called aggregators 

// we are making it pub so other code depends on this can make use of `trait`

// trait can have multiple signiture of methods, one per line, 
// and each line ends with ";"
pub trait Summary {
    fn summarize(&self) -> String; // after fn signature instead of def we use `;`
    
    fn summary_author(&self) -> String;
    
    // default implementation of method on trait.
    fn content(&self) -> String {
        format!("Read more... from ({})", self.summary_author())
    }
}

pub struct NewsArticle {
    pub headline: String, 
    pub content: String, 
    pub location: String, 
    pub author: String, 
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {} ({})", self.headline, self.author, self.location)
    }
    
    fn summary_author(&self) -> String {
        format!("{}", self.author)
    }
}


pub struct Tweet {
    pub username: String, 
    pub content: String, 
    pub reply: String, 
    pub retweet: String, 
}

// impl keyword 
// then trait name 
// for keyword 
// struct name
// inside the brackets we have signature from trait and fill the methods implementation.
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} : {}",self.username, self.content)
    }
    
    fn summary_author(&self) -> String {
        format!("{}", self.username)
    }
}

// Traits can be passed parameter to function let's define `notify` like below with `impl Trait` as syntax.
// Instead of a concrete type for the item parameter, we specify the impl keyword and the trait name. This parameter accepts any type that implements the specified trait.
pub fn notify(item: &impl Summary) {
    println!("Breaking news {}", item.summarize());
}

// TraitBounds.
/* 
    pub fn notify<T: Summary>(item1: &T, item2: &T)
*/

// Specifying multiple trait bounds with `+` syntax.
/* 
    pub fn notify(item: &(impl Summary + Display)) {}
    
    // ... or 
    // The + syntax is also valid with trait bounds on generic types:

    pub fn notify<T: Summary + Display> (item: &T) {}
*/

// Clearer Trait Bounds with where Clauses
/* 
    Using too many trait bounds has its downsides. Each generic has its own trait bounds, 
    so functions with multiple generic type parameters can contain lots of trait bound 
    information between the function’s name and its parameter list, 
    making the function signature hard to read. For this reason, 
    Rust has alternate syntax for specifying trait bounds inside a 
    where clause after the function signature. So, instead of writing this:
    
    pub fn notify<T: Summary + Debug, U: Clone + Display>(item1: &T, item2: &U) 
    
    
    we can make use of where clause. 
    
    pub fn notify<T, U>(item1: &T, item2: &U)  
    where 
        T: Summary + Debug, 
        U: Clone + Display,
    {}
    
    
    /// This function’s signature is less cluttered: the function name, parameter list, 
    /// and return type are close together, 
    /// similar to a function without lots of trait bounds.
*/

// Returning Types That Implement Traits
// we can use impl trait to return type of some value that implement trati.
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("hello"),
        content: String::from("hello"),
        reply: String::from("hello"),
        retweet: String::from("hello"),
    }
}

// The ability to specify a return type only by the trait 
// it implements is especially useful in the context of closures and iterators
// 

/* 
    
    Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations
    and are used extensively in the Rust standard library. 
    For example, the standard library implements the ToString trait on any type that implements 
    the Display trait. The impl block in the standard library looks similar to this code:

    impl<T: Display> ToString for T {
        // --snip--
    }

*/

pub fn main() {
    let tweet = Tweet {
        username: String::from("sahil"),
        content: String::from("Started learning trait from rust!"),
        reply: String::from("5"),
        retweet: String::from("2"),
    };
    
    println!("Summary => {}", tweet.summarize());
    // can't call default implementation over overloaded method.
    println!("Content  => {}", tweet.content());
    
    notify(&tweet);
    
    
}